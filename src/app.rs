use chrono::Duration;
use clap::Subcommand;
use crossterm::event::KeyCode;
use regex::Regex;
use tui::backend::Backend;
use tui::style::Color;
use tui::style::Style;
use tui::Frame;

use self::modes::Clock;
use self::modes::DurationFormat;
use self::modes::Stopwatch;
use self::modes::Timer;

pub(crate) mod modes;

#[derive(Debug, Subcommand)]
pub(crate) enum Mode {
    /// The clock mode displays the current time, the default mode.
    Clock {
        /// Do not show date
        #[clap(short = 'D', long, takes_value = false)]
        no_date: bool,
        /// Show milliseconds
        #[clap(short, long, takes_value = false)]
        millis: bool,
    },
    /// The timer mode displays the remaining time until the timer is finished.
    Timer {
        #[clap(short, long, value_parser = parse_duration, default_value = "5m")]
        duration: Duration,

        /// Hide milliseconds
        #[clap(long = "no-millis", short = 'M', takes_value = false)]
        no_millis: bool,
    },
    /// The stopwatch mode displays the elapsed time since it was started.
    Stopwatch,
}

#[derive(clap::Parser)]
#[clap(name = "tclock", about = "A clock app in terminal", long_about = None)]
pub(crate) struct App {
    #[clap(subcommand)]
    pub mode: Option<Mode>,
    /// Foreground color of the clock, possible values are:
    ///     a) Any one of: Black, Red, Green, Yellow, Blue, Magenta, Cyan, Gray, DarkGray, LightRed, LightGreen, LightYellow, LightBlue, LightMagenta, LightCyan, White.
    ///     b) Hexadecimal color code: #RRGGBB.
    #[clap(short, long, value_parser = parse_color, default_value = "green")]
    pub color: Color,
    /// Size of the clock, should be a positive integer (>=1).
    #[clap(short, long, value_parser, default_value = "1")]
    pub size: u16,

    #[clap(skip)]
    clock: Option<Clock>,
    #[clap(skip)]
    timer: Option<Timer>,
    #[clap(skip)]
    stopwatch: Option<Stopwatch>,
}

impl App {
    pub fn init_app(&mut self) {
        let style = Style::default().fg(self.color);
        let mode = self.mode.as_ref().unwrap_or(&Mode::Clock {
            no_date: false,
            millis: false,
        });
        match mode {
            Mode::Clock { no_date, millis } => {
                self.clock = Some(Clock {
                    size: self.size,
                    style,
                    show_date: !no_date.to_owned(),
                    show_millis: millis.to_owned(),
                });
            }
            Mode::Timer {
                duration,
                no_millis,
            } => {
                let format = if *no_millis {
                    DurationFormat::HourMinSec
                } else {
                    DurationFormat::HourMinSecDeci
                };
                self.timer = Some(Timer::new(duration.to_owned(), self.size, style, format));
            }
            Mode::Stopwatch => {
                self.stopwatch = Some(Stopwatch::new(self.size, style));
            }
        }
    }

    pub fn ui<B: Backend>(&self, f: &mut Frame<B>) {
        if let Some(ref w) = self.clock {
            f.render_widget(w, f.size());
        } else if let Some(ref w) = self.timer {
            f.render_widget(w, f.size());
        } else if let Some(ref w) = self.stopwatch {
            f.render_widget(w, f.size());
        }
    }

    pub fn on_key(&mut self, key: KeyCode) {
        if let Some(_w) = self.clock.as_mut() {
        } else if let Some(w) = self.timer.as_mut() {
            match key {
                KeyCode::Char(' ') => {
                    if w.is_paused() {
                        w.resume();
                    } else {
                        w.pause();
                    }
                }
                _ => {}
            }
        } else if let Some(w) = self.stopwatch.as_mut() {
            match key {
                KeyCode::Char(' ') => {
                    if w.is_paused() {
                        w.resume();
                    } else {
                        w.pause();
                    }
                }
                _ => {}
            }
        }
    }
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    let reg = Regex::new(r"^(\d+)([smhdSMHD])$").unwrap();
    let cap = reg
        .captures(s)
        .ok_or_else(|| format!("{} is not a valid duration", s))?;

    let num = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let unit = cap.get(2).unwrap().as_str().to_lowercase();

    match unit.as_str() {
        "s" => Ok(Duration::seconds(num)),
        "m" => Ok(Duration::minutes(num)),
        "h" => Ok(Duration::hours(num)),
        "d" => Ok(Duration::days(num)),
        _ => Err(format!("Invalid duration: {}", s).into()),
    }
}

fn parse_color(s: &str) -> Result<Color, String> {
    let s = s.to_lowercase();
    let reg = Regex::new(r"^#([0-9a-f]{6})$").unwrap();
    match s.as_str() {
        "black" => Ok(Color::Black),
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "yellow" => Ok(Color::Yellow),
        "blue" => Ok(Color::Blue),
        "magenta" => Ok(Color::Magenta),
        "cyan" => Ok(Color::Cyan),
        "gray" => Ok(Color::Gray),
        "darkgray" => Ok(Color::DarkGray),
        "lightred" => Ok(Color::LightRed),
        "lightGreen" => Ok(Color::LightGreen),
        "lightYellow" => Ok(Color::LightYellow),
        "lightBlue" => Ok(Color::LightBlue),
        "lightMagenta" => Ok(Color::LightMagenta),
        "lightCyan" => Ok(Color::LightCyan),
        "white" => Ok(Color::White),
        s => {
            let cap = reg
                .captures(s)
                .ok_or_else(|| format!("Invalid color: {}", s))?;
            let hex = cap.get(1).unwrap().as_str();
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
            let b = u8::from_str_radix(&hex[4..], 16).unwrap();
            Ok(Color::Rgb(r, g, b))
        }
    }
}
