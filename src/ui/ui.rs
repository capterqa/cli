use crate::{
    workflow::{run_source::RunSource, WorkflowConfig},
    CliOptions,
};
use crossterm::{
    execute,
    style::{Attribute, Print, SetAttribute},
    tty::IsTty,
};
use serde_json::{json, Value};
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
            Print(TerminalUi::format_attr("ci", &json!(&source.ci))),
            Print(TerminalUi::format_attr("repo", &json!(&source.repository))),
            Print(TerminalUi::format_attr("sha", &json!(&source.sha))),
            Print(TerminalUi::format_attr("branch", &json!(&source.branch))),
            Print(format!(
                "  meta: {}\n",
                &source.meta.clone().unwrap_or(Value::Null)
            )),
            Print(TerminalUi::format_attr("commit", &json!(&source.commit))),
            Print(TerminalUi::format_attr("pr", &json!(&source.pr))),
            SetAttribute(Attribute::Reset),
        )
        .unwrap();
    }

    fn format_attr(key: &str, val: &Value) -> String {
        format!("  {}: {}\n", &key, &val.clone())
    }

    pub fn print_init() {
        execute!(
            stdout(),
            SetAttribute(Attribute::Bold),
            Print("\nWelcome to Capter!\n"),
            SetAttribute(Attribute::Reset),
            Print("\nWe've created a workflow in "),
            SetAttribute(Attribute::Underlined),
            Print(".capter/example.test.yml"),
            SetAttribute(Attribute::Reset),
            Print("\n"),
            Print("Run it by calling: "),
            SetAttribute(Attribute::Bold),
            Print("capter test --token [TOKEN]"),
            SetAttribute(Attribute::Reset),
            Print("\n\n"),
            Print("For more information, go to https://docs.capter.io or call: "),
            SetAttribute(Attribute::Bold),
            Print("capter test --help"),
            Print("\n"),
            SetAttribute(Attribute::Reset),
        )
        .unwrap();
    }
}
