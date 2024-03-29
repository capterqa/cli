mod assert;
mod ci;
mod compile;
mod ui;
mod utils;
mod workflow;

use clap::{crate_version, load_yaml, App, AppSettings};
use dotenv::dotenv;
use globwalk;
use serde::Deserialize;
use serde_json::json;
use ui::TerminalUi;
use ureq;
use utils::{exit_with_code, Logger};
use workflow::{workflow_result::WorkflowResult, RunSource, WorkflowConfig};

pub struct CliOptions {
    is_debug: bool,
    timeout: u64,
}

#[derive(Deserialize)]
pub struct WebhookResponse {
    url: String,
}

impl Default for CliOptions {
    fn default() -> CliOptions {
        CliOptions {
            is_debug: false,
            timeout: 30,
        }
    }
}

fn main() {
    // read .env file
    dotenv().ok();

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
        // the timeout for requests
        let timeout = matches.value_of("timeout");

        let cli_options = CliOptions {
            is_debug,
            timeout: timeout.unwrap_or("30").parse().unwrap_or(30),
        };

        // we'll collect all runs in this array so we can post it
        // to the webhook after the run is complete
        let mut workflow_runs: Vec<WorkflowResult> = vec![];

        // collect the source information
        let source = RunSource::new(&cli_options);

        let entries = match globwalk::glob(tests_glob) {
            Ok(res) => res,
            _ => exit_with_code(
                exitcode::USAGE,
                Some(&format!("Invalid search glob: `{}`", tests_glob)),
            ),
        };

        let configs: Vec<WorkflowConfig> = entries
            .map(|entry| {
                let entry = entry.expect("Invalid path");
                let workflow = WorkflowConfig::from_yaml_file(&entry.into_path());
                workflow
            })
            .collect();

        // this sets up our UI
        let mut terminal_ui = TerminalUi::new(&configs, &source, &cli_options);

        let mut passed = true;

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
            let workflow_result =
                WorkflowResult::from_config(&cli_options, &workflow_config, |event| {
                    terminal_ui.update(event);
                });

            if let Ok(workflow_run) = workflow_result {
                if workflow_run.passed == false {
                    passed = false
                }
                workflow_runs.push(workflow_run);
            } else {
                // TODO: handle error
            }
        }

        terminal_ui.summarize(&workflow_runs);

        // write to log on fail
        if !passed {
            let mut logger = Logger::new();
            logger.log_workflow_results(&workflow_runs);
        }

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

            terminal_ui.webhook_start();

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
                Ok(res) => {
                    // we have a response
                    // check for url in response
                    let webhook_response: Option<WebhookResponse> = res.into_json().unwrap_or(None);

                    // render the url so the user can go straight to the run
                    terminal_ui.webhook_done(webhook_response);
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

        if passed {
            exit_with_code(exitcode::OK, None);
        } else {
            exit_with_code(1, None);
        }
    }

    // handle the subcommand `test`
    if matches.subcommand_matches("init").is_some() {
        WorkflowConfig::create_example();
        TerminalUi::print_init();
        exit_with_code(exitcode::OK, None);
    }
}
