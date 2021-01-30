use crate::workflow::{source::Source, WorkflowConfig};
use crossterm::{
    execute,
    style::{Attribute, Print, SetAttribute},
    tty::IsTty,
};
use std::{io::stdout, time::Instant};

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
    pub fn new(configs: &Vec<WorkflowConfig>, is_debug: bool) -> TerminalUi {
        let is_tty = match is_debug {
            true => false,
            false => stdout().is_tty(),
        };

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
            is_debug,
        }
    }

    pub fn print_run_source(&self, source: &Source) {
        if self.is_debug {
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
                SetAttribute(Attribute::Reset),
            )
            .unwrap();
        }
    }

    fn format_attr(key: &str, val: &Option<String>) -> String {
        format!("  {}: {}\n", &key, &val.clone().unwrap_or("".to_string()))
    }
}
