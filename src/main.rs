mod assert;
mod compile;
mod ui;
mod workflow;

use chrono::{DateTime, Utc};
use clap::{crate_version, load_yaml, App, AppSettings};
use globwalk;
use path_clean::PathClean;
use serde::Serialize;
use serde_json::json;
use std::fs;
use std::time::Instant;
use ui::TerminalUi;
use ureq;
use workflow::{get_source, parse_yaml, run_workflow, RequestData, WorkflowConfig};

#[derive(Debug, Serialize)]
pub struct WorkflowRun {
    pub file: Option<String>,
    pub name: String,
    pub workflow: WorkflowConfig,
    pub created_at: DateTime<Utc>,
    pub run_time: i64,
    pub passed: bool,
    pub requests: Vec<RequestData>,
}

fn main() {
    let yml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(crate_version!())
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("test") {
        let is_debug = matches.is_present("debug");
        let token = matches.value_of("token");
        let webhook = matches.value_of("webhook");
        let glob_pattern = matches.value_of("INPUT").unwrap();
        let dry_run = matches.is_present("dry-run");

        let mut workflow_runs: Vec<WorkflowRun> = vec![];
        let source = get_source();

        let entries = globwalk::glob(glob_pattern).expect("Failed to read glob pattern");
        let configs: Vec<WorkflowConfig> = entries
            .map(|entry| {
                let path = entry.unwrap().into_path().clean();
                let path = format!("{}", path.display());
                let content = fs::read_to_string(path.clone()).unwrap();
                let workflow = parse_yaml(content, path).expect("Failed to parse config.");
                workflow
            })
            .collect();

        let mut terminal_ui = TerminalUi::new(&configs, is_debug);

        for workflow_config in configs {
            if workflow_config.skip.is_some() {
                terminal_ui.skipped_workflow(&workflow_config);
                continue;
            }

            // track time
            let timer = Instant::now();

            let (requests, passed) = run_workflow(&workflow_config, |event| {
                terminal_ui.update(event);
            })
            .unwrap();

            // save time
            let run_time = timer.elapsed().as_millis() as i64;

            let workflow_run = WorkflowRun {
                name: workflow_config.name.clone(),
                file: Some(workflow_config.file.clone().unwrap_or("".to_string())),
                workflow: workflow_config,
                created_at: Utc::now(),
                requests,
                run_time,
                passed,
            };

            workflow_runs.push(workflow_run);
        }

        terminal_ui.summarize(&workflow_runs);

        let webhook = match webhook {
            Some(val) => Some(val),
            // if no value, then check if token is set
            // if token is set, we send to capter.io
            None => match token.is_some() {
                true => Some("https://app.capter.io/api/webhooks/runs"),
                false => None,
            },
        };

        // post to webhook
        if let Some(webhook) = webhook {
            if dry_run {
                terminal_ui.dry_run();
                return;
            }

            terminal_ui.webhook_start(webhook);

            let mut request = ureq::request("POST", webhook);

            // add token if set
            if let Some(token) = token {
                request = request.query("token", token);
            }

            let response = request.send_json(json!({
                "source": json!(source),
                "data": json!(workflow_runs)
            }));

            match response {
                Ok(_) => {
                    terminal_ui.webhook_done();
                }
                Err(ureq::Error::Status(_, res)) => {
                    let error = res.into_string().unwrap();
                    terminal_ui.webhook_error(&error);
                }
                Err(err) => {
                    terminal_ui.webhook_error(&err.to_string());
                }
            }
        }
    }
}
