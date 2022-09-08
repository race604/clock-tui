use std::{cell::RefCell, cmp::min, process::Command};

use crate::clock_text::BricksText;
use chrono::{DateTime, Duration, Local};
use tui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use crate::app::Pause;

use super::{format_duration, render_centered, DurationFormat};

pub struct Timer {
    pub size: u16,
    pub style: Style,
    pub repeat: bool,
    pub durations: Vec<Duration>,
    pub titles: Vec<String>,
    pub execute: Vec<String>,
    format: DurationFormat,
    passed: Duration,
    started_at: Option<DateTime<Local>>,
    execute_result: RefCell<Option<String>>,
}

impl Timer {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        size: u16,
        style: Style,
        durations: Vec<Duration>,
        titles: Vec<String>,
        repeat: bool,
        format: DurationFormat,
        paused: bool,
        execute: Vec<String>,
    ) -> Self {
        Self {
            size,
            style,
            durations,
            titles,
            repeat,
            execute,
            format,
            passed: Duration::zero(),
            started_at: (!paused).then(Local::now),
            execute_result: RefCell::new(None),
        }
    }

    pub(crate) fn remaining_time(&self) -> (Duration, usize) {
        let total_passed = if let Some(started_at) = self.started_at {
            self.passed + (Local::now() - started_at)
        } else {
            self.passed
        };

        let mut idx = 0;
        let mut next_checkpoint = self.durations[idx];
        while next_checkpoint < total_passed {
            if idx >= self.durations.len() - 1 && !self.repeat {
                break;
            }
            idx = (idx + 1) % self.durations.len();
            next_checkpoint = next_checkpoint + self.durations[idx];
        }

        (next_checkpoint - total_passed, idx)
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
        let (remaining_time, idx) = self.remaining_time();
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

        let header = if self.titles.is_empty() {
            None
        } else {
            Some(self.titles[min(idx, self.titles.len() - 1)].clone())
        };

        let text = BricksText::new(time_str.as_str(), self.size, self.size, self.style);
        let footer = if self.is_paused() {
            Some("PAUSED (press <SPACE> to resume)".to_string())
        } else {
            self.execute_result.borrow().clone()
        };
        render_centered(area, buf, &text, header, footer);
    }
}

impl Pause for Timer {
    fn is_paused(&self) -> bool {
        self.started_at.is_none()
    }

    fn pause(&mut self) {
        if let Some(started_at) = self.started_at {
            self.passed = self.passed + (Local::now() - started_at);
            self.started_at = None;
        }
    }

    fn resume(&mut self) {
        if self.started_at.is_none() {
            self.started_at = Some(Local::now());
        }
    }
}
