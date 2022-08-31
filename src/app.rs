use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::TimeZone;
use clap::Subcommand;
use crossterm::event::KeyCode;
use regex::Regex;
use tui::backend::Backend;
use tui::style::Color;
use tui::style::Style;
use tui::Frame;

use self::modes::Clock;
use self::modes::Countdown;
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
        /// Do not show seconds
        #[clap(short = 'S', long, takes_value = false)]
        no_seconds: bool,
        /// Show milliseconds
        #[clap(short, long, takes_value = false)]
        millis: bool,
    },
    /// The timer mode displays the remaining time until the timer is finished.
    Timer {
        /// Initial duration for timer, value can be 10s for 10 seconds, 1m for 1 minute, etc.
        #[clap(short, long, value_parser = parse_duration, default_value = "5m")]
        duration: Duration,

        /// Hide milliseconds
        #[clap(long = "no-millis", short = 'M', takes_value = false)]
        no_millis: bool,

        /// Start the timer paused
        #[clap(long = "paused", short = 'P', takes_value = false)]
        paused: bool,

        /// Command to run when the timer ends
        #[clap(long, short, multiple = true, allow_hyphen_values = true)]
        execute: Vec<String>,
    },
    /// The stopwatch mode displays the elapsed time since it was started.
    Stopwatch,
    /// The countdown timer mode shows the duration to a specific time
    Countdown {
        /// The target time to countdown to, eg. "2023-01-01", or "20:00"
        #[clap(long, short, value_parser = parse_datetime)]
        time: DateTime<Local>,

        /// Title or description for countdown show in header
        #[clap(long, short = 'T')]
        title: Option<String>,

        /// Continue to countdown after pass the target time
        #[clap(long = "continue", short = 'c', takes_value = false)]
        continue_on_zero: bool,

        /// Reverse the countdown, a.k.a. countup
        #[clap(long, short, takes_value = false)]
        reverse: bool,

        /// Show milliseconds
        #[clap(short, long, takes_value = false)]
        millis: bool,
    },
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
    #[clap(skip)]
    countdown: Option<Countdown>,
}

/// Trait for widgets that can be paused
pub(crate) trait Pause {
    fn is_paused(&self) -> bool;

    fn pause(&mut self);

    fn resume(&mut self);

    fn toggle_paused(&mut self) {
        if self.is_paused() {
            self.resume()
        } else {
            self.pause()
        }
    }
}

impl App {
    pub fn init_app(&mut self) {
        let style = Style::default().fg(self.color);
        let mode = self.mode.as_ref().unwrap_or(&Mode::Clock {
            no_date: false,
            millis: false,
            no_seconds: false,
        });
        match mode {
            Mode::Clock {
                no_date,
                no_seconds,
                millis,
            } => {
                self.clock = Some(Clock {
                    size: self.size,
                    style,
                    show_date: !no_date,
                    show_millis: *millis,
                    show_secs: !no_seconds,
                });
            }
            Mode::Timer {
                duration,
                no_millis,
                paused,
                execute,
            } => {
                let format = if *no_millis {
                    DurationFormat::HourMinSec
                } else {
                    DurationFormat::HourMinSecDeci
                };
                self.timer = Some(Timer::new(
                    *duration,
                    self.size,
                    style,
                    format,
                    *paused,
                    execute.to_owned(),
                ));
            }
            Mode::Stopwatch => {
                self.stopwatch = Some(Stopwatch::new(self.size, style));
            }
            Mode::Countdown {
                time,
                title,
                continue_on_zero,
                reverse,
                millis,
            } => {
                self.countdown = Some(Countdown {
                    size: self.size,
                    style,
                    time: *time,
                    title: title.to_owned(),
                    continue_on_zero: *continue_on_zero,
                    reverse: *reverse,
                    format: if *millis {
                        DurationFormat::HourMinSecDeci
                    } else {
                        DurationFormat::HourMinSec
                    },
                })
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
        } else if let Some(ref w) = self.countdown {
            f.render_widget(w, f.size());
        }
    }

    pub fn on_key(&mut self, key: KeyCode) {
        if let Some(_w) = self.clock.as_mut() {
        } else if let Some(w) = self.timer.as_mut() {
            handle_key(w, key);
        } else if let Some(w) = self.stopwatch.as_mut() {
            handle_key(w, key);
        }
    }
}

fn handle_key<T: Pause>(widget: &mut T, key: KeyCode) {
    if let KeyCode::Char(' ') = key {
        widget.toggle_paused()
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
        _ => Err(format!("Invalid duration: {}", s)),
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
        "lightgreen" => Ok(Color::LightGreen),
        "lightyellow" => Ok(Color::LightYellow),
        "lightblue" => Ok(Color::LightBlue),
        "lightmagenta" => Ok(Color::LightMagenta),
        "lightcyan" => Ok(Color::LightCyan),
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

fn parse_datetime(s: &str) -> Result<DateTime<Local>, String> {
    let s = s.trim();
    let today = Local::today();

    let time = NaiveTime::parse_from_str(s, "%H:%M:%S");
    if time.is_ok() {
        let time = NaiveDateTime::new(today.naive_local(), time.unwrap());
        return Ok(Local.from_local_datetime(&time).unwrap());
    }

    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d");
    if date.is_ok() {
        let time = NaiveDateTime::new(date.unwrap(), NaiveTime::from_hms(0, 0, 0));
        return Ok(Local.from_local_datetime(&time).unwrap());
    }

    let date_time = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S");
    if date_time.is_ok() {
        return Ok(Local.from_local_datetime(&date_time.unwrap()).unwrap());
    }

    let rfc_time = DateTime::parse_from_rfc3339(s);
    if rfc_time.is_ok() {
        return Ok(rfc_time.unwrap().with_timezone(&Local));
    }

    return Err("Invalid time format".to_string());
}
