use std::{cell::RefCell, process::Command};

use chrono::{DateTime, Duration, Local};
use clock_tui::bricks_text::BricksText;
use tui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use crate::app::Pause;

use super::{format_duration, render_centered, DurationFormat};

pub struct Timer {
    pub size: u16,
    pub style: Style,
    pub execute: Vec<String>,
    format: DurationFormat,
    duration: Duration,
    ended_at: Option<DateTime<Local>>,
    execute_result: RefCell<Option<String>>,
}

impl Timer {
    pub(crate) fn new(
        duration: Duration,
        size: u16,
        style: Style,
        format: DurationFormat,
        paused: bool,
        execute: Vec<String>,
    ) -> Self {
        Self {
            duration,
            size,
            execute,
            style,
            format,
            ended_at: (!paused).then(|| Local::now() + duration),
            execute_result: RefCell::new(None),
        }
    }

    pub(crate) fn remaining_time(&self) -> Duration {
        if let Some(end_at) = self.ended_at {
            let now = Local::now();
            end_at.signed_duration_since(now)
        } else {
            self.duration
        }
    }
}

fn execute(execute: &[String]) -> String {
    let mut cmd = Command::new("sh");
    cmd.arg("-c");
    let cmd_str = execute.join(" ");
    cmd.arg(cmd_str);
    let output = cmd.output();
    match output {
        Ok(output) => {
            if !output.status.success() {
                format!("[ERROR] {}", String::from_utf8_lossy(&output.stderr))
            } else {
                format!("[SUCCEED] {}", String::from_utf8_lossy(&output.stdout))
            }
        }
        Err(e) => {
            format!("[FAILED] {}", e)
        }
    }
}

impl Widget for &Timer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let remaining_time = self.remaining_time();
        let time_str = if remaining_time < Duration::zero() {
            if !self.execute.is_empty() && self.execute_result.borrow().is_none() {
                let result = execute(&self.execute);
                *self.execute_result.borrow_mut() = Some(result);
            }
            if (remaining_time.num_milliseconds()).abs() % 1000 < 500 {
                return;
            } else {
                format_duration(Duration::zero(), self.format)
            }
        } else {
            format_duration(remaining_time, self.format)
        };

        let text = BricksText::new(time_str.as_str(), self.size, self.size, self.style);
        let footer = if self.is_paused() {
            Some("PAUSED (press <SPACE> to resume)".to_string())
        } else {
            self.execute_result.borrow().clone()
        };
        render_centered(area, buf, &text, None, footer);
    }
}

impl Pause for Timer {
    fn is_paused(&self) -> bool {
        self.ended_at.is_none()
    }

    fn pause(&mut self) {
        if let Some(end_at) = self.ended_at {
            if end_at <= Local::now() {
                self.duration = Duration::zero();
            } else {
                self.duration = end_at - Local::now();
                self.ended_at = None;
            }
        }
    }

    fn resume(&mut self) {
        if self.ended_at.is_none() {
            self.ended_at = Some(Local::now() + self.duration);
        }
    }
}
