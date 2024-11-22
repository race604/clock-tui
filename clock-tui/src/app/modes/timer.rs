use std::{cell::RefCell, cmp::min, process::Command};

use crate::clock_text::BricksText;
use chrono::{DateTime, Duration, Local};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};
use crate::app::Pause;

use super::{format_duration, render_centered, DurationFormat};

pub struct Timer {
    pub size: u16,
    pub style: Style,
    pub repeat: bool,
    pub durations: Vec<Duration>,
    pub titles: Vec<String>,
    pub execute: Vec<String>,
    auto_quit: bool,
    format: DurationFormat,
    passed: Duration,
    started_at: Option<DateTime<Local>>,
    execute_result: RefCell<Option<String>>,
    flash_state: RefCell<bool>, // Add this new field
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
        auto_quit: bool,
        execute: Vec<String>,
    ) -> Self {
        Self {
            size,
            style,
            durations,
            titles,
            repeat,
            execute,
            auto_quit,
            format,
            passed: Duration::zero(),
            started_at: (!paused).then(Local::now),
            execute_result: RefCell::new(None),
            flash_state: RefCell::new(false), // Initialize the new field
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

    pub(crate) fn is_finished(&self) -> bool {
        return self.auto_quit && !self.execute_result.borrow().is_none();
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
        
        if remaining_time < Duration::zero() {
            if self.execute_result.borrow().is_none() {
                if !self.execute.is_empty() {
                    let result = execute(&self.execute);
                    *self.execute_result.borrow_mut() = Some(result);
                } else {
                    *self.execute_result.borrow_mut() = Some("".to_owned())
                }
            }

            // Flash the screen when timer is done
            let should_flash = remaining_time.num_milliseconds().abs() % 1000 < 500;
            *self.flash_state.borrow_mut() = should_flash;

            // Fill the entire area with the flash color
            let flash_style = if *self.flash_state.borrow() {
                Style::default().bg(Color::Green)
            } else {
                Style::default().bg(Color::Black)
            };

            // Fill the entire area with the flash color
            for y in area.top()..area.bottom() {
                for x in area.left()..area.right() {
                    buf.get_mut(x, y).set_style(flash_style);
                }
            }

            // Only render the text during the visible phase
            if should_flash {
                let time_str = format_duration(Duration::zero(), self.format);
                let header = if self.titles.is_empty() {
                    None
                } else {
                    Some(self.titles[min(idx, self.titles.len() - 1)].clone())
                };

                let text = BricksText::new(
                    time_str.as_str(),
                    self.size,
                    self.size,
                    self.style.fg(Color::Black), // Make text visible on green background
                );

                let footer = if self.is_paused() {
                    Some("PAUSED (press <SPACE> to resume)".to_string())
                } else {
                    self.execute_result.borrow().clone()
                };

                render_centered(area, buf, &text, header, footer);
            }
            if !should_flash {
                let time_str = format_duration(Duration::zero(), self.format);
                let header = if self.titles.is_empty() {
                    None
                } else {
                    Some(self.titles[min(idx, self.titles.len() - 1)].clone())
                };

                let text = BricksText::new(
                    time_str.as_str(),
                    self.size,
                    self.size,
                    self.style, // Use original style during black phase
                );

                let footer = if self.is_paused() {
                    Some("PAUSED (press <SPACE> to resume)".to_string())
                } else {
                    self.execute_result.borrow().clone()
                };

                render_centered(area, buf, &text, header, footer);
            }
        } else {
            // Normal timer display when counting down
            let time_str = format_duration(remaining_time, self.format);
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
