use crate::ui::TerminalUi;
use crossterm::{
    execute,
    style::{Attribute, Color, Print, SetAttribute, SetForegroundColor},
};
use std::io::stdout;

impl TerminalUi {
    pub fn dry_run(&self) {
        execute!(
            stdout(),
            SetAttribute(Attribute::Dim),
            Print("\n---\n\n"),
            Print("dry run - skipped posting to webhook"),
            Print("\n"),
            SetAttribute(Attribute::Reset),
        )
        .unwrap();
    }

    pub fn webhook_start(&self, url: &str) {
        execute!(
            stdout(),
            SetAttribute(Attribute::Dim),
            Print("\n---\n\n"),
            Print(format!("Posting to webhook [{}]... ", url)),
            SetAttribute(Attribute::Reset),
        )
        .unwrap();
    }

    pub fn webhook_done(&self) {
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print("✓\n"),
            SetAttribute(Attribute::Reset),
            SetAttribute(Attribute::Dim),
            Print("done! "),
            SetAttribute(Attribute::Reset),
            Print("✨"),
            Print("\n"),
            SetAttribute(Attribute::Reset),
        )
        .unwrap();
    }
    pub fn webhook_error(&self, error: &str) {
        execute!(
            stdout(),
            SetForegroundColor(Color::Red),
            Print("✕\n"),
            SetAttribute(Attribute::Reset),
            SetAttribute(Attribute::Dim),
            Print("Error sending webhook: "),
            Print(error),
            Print("\n"),
            SetAttribute(Attribute::Reset),
        )
        .unwrap();
    }
}
