use crate::{ui::TerminalUi, workflow::WorkflowConfig};
use crossterm::{
    execute,
    style::{Attribute, Print, SetAttribute},
};
use std::io::stdout;

impl TerminalUi {
    pub fn skipped_workflow(&mut self, config: &WorkflowConfig) {
        let file = config.file.clone().unwrap().clone();
        execute!(
            stdout(),
            SetAttribute(Attribute::Underlined),
            SetAttribute(Attribute::Dim),
            Print("\n"),
            Print(format!("{} [skipped]", file)),
            Print("\n"),
            SetAttribute(Attribute::Reset),
        )
        .unwrap();

        self.skipped_workflows_count += 1;
    }
}
