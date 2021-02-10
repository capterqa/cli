use crate::{ui::TerminalUi, WorkflowResult};
use crossterm::{
    execute,
    style::{Attribute, Color, Print, SetAttribute, SetForegroundColor},
};
use std::io::stdout;

impl TerminalUi {
    pub fn summarize(&self, workflow_runs: &Vec<WorkflowResult>) {
        // find failed assertion
        let mut assertion_results = vec![];
        workflow_runs.iter().for_each(|run| {
            run.requests.iter().for_each(|req| {
                if let Some(res) = &req.response {
                    for result in &res.assertion_results {
                        if !result.passed {
                            assertion_results.push((run, req, result));
                        }
                    }
                }
            })
        });

        for assertion in &assertion_results {
            execute!(
                stdout(),
                Print("\n\n"),
                SetForegroundColor(Color::Red),
                Print(format!(
                    " ▶ {} → {} → {}",
                    assertion.0.workflow.name, &assertion.1.name, &assertion.2.assertion.property,
                )),
                SetAttribute(Attribute::Reset),
            )
            .unwrap();

            if let Some(message) = &assertion.2.message {
                execute!(
                    stdout(),
                    Print("\n\n   "),
                    SetAttribute(Attribute::Dim),
                    Print(format!("{}", message)),
                    SetAttribute(Attribute::Reset),
                )
                .unwrap();
            }
        }

        if assertion_results.len() > 0 {
            execute!(stdout(), Print("\n\n")).unwrap();
        }

        self.print_summary();
    }

    pub fn print_summary(&self) {
        execute!(stdout(), Print("\n")).unwrap();

        self.print_summary_numbers(
            "Workflows",
            self.workflow_count,
            self.passed_workflows_count,
            self.failed_workflows_count,
            self.skipped_workflows_count,
        );

        execute!(stdout(), Print("\n")).unwrap();

        self.print_summary_numbers(
            "Requests",
            self.step_count,
            self.passed_steps_count,
            self.failed_steps_count,
            0,
        );

        execute!(stdout(), Print("\n")).unwrap();

        let elapsed = format!("{:.1}", self.timer.elapsed().as_millis() as f32 / 1000.0);
        execute!(
            stdout(),
            SetAttribute(Attribute::Bold),
            Print("Time: "),
            SetAttribute(Attribute::Reset),
            SetAttribute(Attribute::Dim),
            Print(format!("{} s", elapsed)),
        )
        .unwrap();

        execute!(stdout(), Print("\n"), SetAttribute(Attribute::Reset)).unwrap();
    }

    fn print_summary_numbers(
        &self,
        title: &str,
        total: i32,
        passed: i32,
        failed: i32,
        skipped: i32,
    ) {
        execute!(
            stdout(),
            SetAttribute(Attribute::Bold),
            Print(format!("{}: ", title)),
            SetAttribute(Attribute::Reset),
        )
        .unwrap();

        if failed > 0 {
            execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(format!("{} failed", failed)),
                SetAttribute(Attribute::Reset)
            )
            .unwrap();
        }

        if passed > 0 && failed > 0 {
            execute!(stdout(), Print(", ")).unwrap();
        }

        if passed > 0 {
            execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(format!("{} passed", passed)),
                SetAttribute(Attribute::Reset)
            )
            .unwrap();
        }

        if (passed > 0 || failed > 0) && skipped > 0 {
            execute!(stdout(), Print(", ")).unwrap();
        }

        if skipped > 0 {
            execute!(
                stdout(),
                SetForegroundColor(Color::Grey),
                Print(format!("{} skipped", skipped)),
                SetAttribute(Attribute::Reset)
            )
            .unwrap();
        }

        let total = match passed + failed + skipped {
            0 => format!("0 of {} total", total),
            _ => format!(", {} total", total),
        };

        execute!(
            stdout(),
            SetAttribute(Attribute::Dim),
            Print(total),
            SetAttribute(Attribute::Reset)
        )
        .unwrap();
    }
}
