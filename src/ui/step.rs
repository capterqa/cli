use crate::{
    assert::AssertionResultData,
    ui::{StepStatus, TerminalUi},
    workflow::{WorkflowConfig, WorkflowConfigStep},
};
use crossterm::{
    execute,
    style::{Attribute, Color, Colors, Print, SetAttribute, SetColors, SetForegroundColor},
};
use std::io::stdout;

impl TerminalUi {
    pub fn print_step(
        &self,
        config: &WorkflowConfig,
        step: &WorkflowConfigStep,
        status: StepStatus,
    ) {
        let status_str = match status {
            StepStatus::Running => (" RUNS ", Color::Yellow),
            StepStatus::Done(passed) => match passed {
                true => (" PASS ", Color::Green),
                false => (" FAIL ", Color::Red),
            },
            StepStatus::Skipped => (" SKIP ", Color::Grey),
        };

        execute!(
            stdout(),
            SetColors(Colors::new(
                Color::Rgb {
                    r: 50,
                    g: 50,
                    b: 50
                },
                status_str.1
            )),
            Print(status_str.0),
            SetAttribute(Attribute::Reset),
            Print(format!(" {} → {}", config.name, step.name)),
            Print("\n")
        )
        .unwrap();
    }

    pub fn print_assertions(&self, assertions: &Vec<AssertionResultData>) {
        for assertion in assertions {
            let icon = match assertion.passed {
                true => (Color::Green, "✓"),
                false => (Color::Red, "✕"),
            };

            execute!(
                stdout(),
                Print("\n    "),
                SetForegroundColor(icon.0),
                Print(icon.1),
                SetAttribute(Attribute::Reset),
                SetAttribute(Attribute::Dim),
                Print(format!(
                    " {} {} {}",
                    assertion.assertion.property,
                    assertion.assertion.test,
                    assertion.assertion.value.as_str().unwrap_or(""),
                )),
                SetAttribute(Attribute::Reset),
            )
            .unwrap();
        }
    }
}
