mod assert;
mod compile;
mod ui;
mod utils;
mod workflow;

use clap::{crate_version, load_yaml, App, AppSettings};
use globwalk;
use serde_json::json;
use ui::TerminalUi;
use ureq;
use workflow::{workflow_result::WorkflowResult, RunSource, WorkflowConfig};

fn main() {
    let yml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(crate_version!())
        .get_matches();

    // handle the subcommand `test`
    if let Some(matches) = matches.subcommand_matches("test") {
        // will run the CLI in debug mode
        let is_debug = matches.is_present("debug");
        // passing a token will submit the run to the webhook
        let token = matches.value_of("token");
        // the url we will post the run to after completion
        let webhook = matches.value_of("webhook");
        // where to look for the yaml files
        let tests_glob = matches.value_of("INPUT").unwrap();
        // stops the cli from posting to the webhook
        let dry_run = matches.is_present("dry-run");
        // stops the cli from collecting data (branch, commit message) from git
        let skip_git = matches.is_present("skip-git");

        // we'll collect all runs in this array so we can post it
        // to the webhook after the run is complete
        let mut workflow_runs: Vec<WorkflowResult> = vec![];

        // collect the source information
        let source = RunSource::new(skip_git);

        let entries = globwalk::glob(tests_glob).expect("Failed to read glob pattern");
        let configs: Vec<WorkflowConfig> = entries
            .map(|entry| {
                let entry = entry.expect("Invalid path");
                let workflow = WorkflowConfig::from_yaml_file(&entry.into_path());
                workflow
            })
            .collect();

        // this sets up our UI
        let mut terminal_ui = TerminalUi::new(&configs, &source, is_debug);

        for workflow_config in configs {
            // setting `skip: true` in the workflow will stop
            // it from running:
            if workflow_config.skip.is_some() {
                terminal_ui.skipped_workflow(&workflow_config);
                continue;
            }

            // run the workflow and use the callback to update the UI on events like
            // new step, step completed etc.
            // we get `RequestData` back, which is the results of of this workflow
            let workflow_result = WorkflowResult::from_config(&workflow_config, |event| {
                terminal_ui.update(event);
            });

            if let Ok(workflow_run) = workflow_result {
                workflow_runs.push(workflow_run);
            } else {
                // TODO: handle error
            }
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
