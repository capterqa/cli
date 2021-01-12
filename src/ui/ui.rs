use crate::workflow::WorkflowConfig;
use crossterm::tty::IsTty;
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
        }
    }
}
