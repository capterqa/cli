use crate::{
    workflow::{run_source::RunSource, WorkflowConfig},
    CliOptions,
};
use crossterm::{
    execute,
    style::{Attribute, Print, SetAttribute},
    tty::IsTty,
};
use serde_json::Value;
use std::{io::stdout, time::Instant};

/// The `TerminalUi` is responsible for printing out information
/// about the run, its worfklows and requests.
///
/// It can run in either with *tty* or without it. If tty is supported
/// the UI will be a lot nicer, but not all environments support it.
pub struct TerminalUi {
    pub is_tty: bool,
    pub workflow_count: i32,
    pub step_count: i32,
    pub passed_steps_count: i32,
    pub passed_workflows_count: i32,
    pub skipped_workflows_count: i32,
    pub failed_steps_count: i32,
    pub failed_workflows_count: i32,
    pub timer: Instant,
    pub is_debug: bool,
}

impl TerminalUi {
    pub fn new(
        configs: &Vec<WorkflowConfig>,
        source: &RunSource,
        cli_options: &CliOptions,
    ) -> TerminalUi {
        let is_tty = match cli_options.is_debug {
            true => false,
            false => stdout().is_tty(),
        };

        if cli_options.is_debug {
            TerminalUi::print_run_source(source);
        }

        let timer = Instant::now();

        // calculate number of workflows and steps
        let workflow_count = configs.len() as i32;
        let step_count = configs
            .iter()
            .fold(0, |sum, config| sum + config.steps.len()) as i32;

        TerminalUi {
            is_tty,
            timer,
            passed_steps_count: 0,
            passed_workflows_count: 0,
            failed_steps_count: 0,
            failed_workflows_count: 0,
            skipped_workflows_count: 0,
            workflow_count,
            step_count,
            is_debug: cli_options.is_debug,
        }
    }

    fn print_run_source(source: &RunSource) {
        execute!(
            stdout(),
            SetAttribute(Attribute::Bold),
            Print("\ngit info:\n"),
            SetAttribute(Attribute::Reset),
            Print(format!("  source: {:?}\n", &source.source)),
            Print(TerminalUi::format_attr("ci", &source.ci)),
            Print(TerminalUi::format_attr("repo", &source.repository)),
            Print(TerminalUi::format_attr("sha", &source.sha)),
            Print(TerminalUi::format_attr("branch", &source.branch)),
            Print(TerminalUi::format_attr("commit", &source.commit_message)),
            Print(format!(
                "  meta: {}\n",
                &source.meta.clone().unwrap_or(Value::Null)
            )),
            SetAttribute(Attribute::Reset),
        )
        .unwrap();
    }

    fn format_attr(key: &str, val: &Option<String>) -> String {
        format!("  {}: {}\n", &key, &val.clone().unwrap_or("".to_string()))
    }
}
