use crate::{ui::TerminalUi, workflow::run::CallbackEvent};
use crossterm::{
    cursor, execute,
    style::{Attribute, Print, SetAttribute},
    terminal,
    terminal::ClearType,
};
use std::io::stdout;

pub enum StepStatus {
    Running,
    Done(bool),
    Skipped,
}

impl TerminalUi {
    pub fn update(&mut self, event: CallbackEvent) {
        match event {
            CallbackEvent::RunStart(config) => {
                let file = config.file.clone().unwrap().clone();
                execute!(
                    stdout(),
                    SetAttribute(Attribute::Underlined),
                    SetAttribute(Attribute::Dim),
                    Print("\n"),
                    Print(file),
                    Print("\n\n"),
                    SetAttribute(Attribute::Reset),
                )
                .unwrap();
            }
            CallbackEvent::StepStart(config, index) => {
                let step = config.steps[index as usize].clone();

                if self.is_tty {
                    execute!(stdout(), cursor::SavePosition).unwrap();
                    &self.print_step(config, &step, StepStatus::Running);
                    &self.print_summary();
                }
            }
            CallbackEvent::StepDone(config, index, assertion_results, passed) => {
                let step = config.steps[index as usize].clone();

                if self.is_tty {
                    execute!(
                        stdout(),
                        cursor::RestorePosition,
                        terminal::Clear(ClearType::FromCursorDown)
                    )
                    .unwrap();
                }

                &self.print_step(config, &step, StepStatus::Done(passed));

                if !passed {
                    &self.print_assertions(assertion_results);
                    execute!(stdout(), Print("\n")).unwrap();
                    if config.steps.len() - 1 > index as usize {
                        execute!(stdout(), Print("\n")).unwrap();
                    }
                }

                match passed {
                    true => self.passed_steps_count += 1,
                    false => self.failed_steps_count += 1,
                }
            }
            CallbackEvent::RunDone(_config, passed) => match passed {
                true => self.passed_workflows_count += 1,
                false => self.failed_workflows_count += 1,
            },
            CallbackEvent::StepSkipped(config, index) => {
                let step = config.steps[index as usize].clone();

                if self.is_tty {
                    execute!(
                        stdout(),
                        cursor::RestorePosition,
                        terminal::Clear(ClearType::FromCursorDown)
                    )
                    .unwrap();
                }

                &self.print_step(config, &step, StepStatus::Skipped);
            }
        }
    }
}
