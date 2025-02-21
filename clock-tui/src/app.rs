use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::TimeZone;
use chrono_tz::Tz;
use clap::Subcommand;
use crossterm::event::KeyCode;
use regex::Regex;
use ratatui::{Frame, style::{Color, Style}};

use self::modes::Clock;
use self::modes::Countdown;
use self::modes::DurationFormat;
use self::modes::Stopwatch;
use self::modes::Timer;
use self::modes::Pause;

pub mod modes;

#[derive(Debug, Subcommand)]
pub enum Mode {
    /// The clock mode displays the current time, the default mode.
    Clock {
        /// Custome timezone, for example "America/New_York", use local timezone if not specificed
        #[clap(short = 'z', long, value_parser=parse_timezone)]
        timezone: Option<Tz>,
        /// Do not show date
        #[clap(short = 'D', long, action)]
        no_date: bool,
        /// Do not show seconds
        #[clap(short = 'S', long, action)]
        no_seconds: bool,
        /// Show milliseconds
        #[clap(short, long, action)]
        millis: bool,
    },
    /// The timer mode displays the remaining time until the timer is finished.
    Timer {
        /// Initial duration for timer, value can be 10s for 10 seconds, 1m for 1 minute, etc.
        /// Also accept mulitple duration value and run the timers sequentially, eg. 25m 5m
        #[clap(short, long="duration", value_parser = parse_duration, num_args = 1.., default_value = "5m")]
        durations: Vec<Duration>,

        /// Set the title for the timer, also accept mulitple titles for each durations correspondingly
        #[clap(short, long = "title", num_args = 0..)]
        titles: Vec<String>,

        /// Restart the timer when timer is over
        #[clap(long, short, action)]
        repeat: bool,

        /// Hide milliseconds
        #[clap(long = "no-millis", short = 'M', action)]
        no_millis: bool,

        /// Start the timer paused
        #[clap(long = "paused", short = 'P', action)]
        paused: bool,

        /// Auto quit when time is up
        #[clap(long = "quit", short = 'Q', action)]
        auto_quit: bool,

        /// Command to run when the timer ends
        #[clap(long, short, num_args = 1.., allow_hyphen_values = true)]
        execute: Vec<String>,
    },
    /// The stopwatch mode displays the elapsed time since it was started.
    Stopwatch,
    /// The countdown timer mode shows the duration to a specific time
    Countdown {
        /// The target time to countdown to, eg. "2023-01-01", "20:00", "2022-12-25 20:00:00" or "2022-12-25T20:00:00-04:00"
        #[clap(long, short, value_parser = parse_datetime)]
        time: DateTime<Local>,

        /// Title or description for countdown show in header
        #[clap(long, short = 'T')]
        title: Option<String>,

        /// Continue to countdown after pass the target time
        #[clap(long = "continue", short = 'c', action)]
        continue_on_zero: bool,

        /// Reverse the countdown, a.k.a. countup
        #[clap(long, short, action)]
        reverse: bool,

        /// Show milliseconds
        #[clap(short, long, action)]
        millis: bool,
    },
}

use crate::config::Config;

#[derive(clap::Parser, Default)]
#[clap(name = "tclock", about = "A clock app in terminal", long_about = None)]
pub struct App {
    #[clap(subcommand)]
    pub mode: Option<Mode>,
    /// Foreground color of the clock, possible values are:
    ///     a) Any one of: Black, Red, Green, Yellow, Blue, Magenta, Cyan, Gray, DarkGray, LightRed, LightGreen, LightYellow, LightBlue, LightMagenta, LightCyan, White.
    ///     b) Hexadecimal color code: #RRGGBB.
    #[clap(short, long, value_parser = parse_color)]
    pub color: Option<Color>,
    /// Size of the clock, should be a positive integer (>=1).
    #[clap(short, long, value_parser)]
    pub size: Option<u16>,

    #[clap(skip)]
    clock: Option<Clock>,
    #[clap(skip)]
    timer: Option<Timer>,
    #[clap(skip)]
    stopwatch: Option<Stopwatch>,
    #[clap(skip)]
    countdown: Option<Countdown>,
}

impl App {
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = Some(mode);
        self.init_app();
    }

    pub fn init_app(&mut self) {
        // Load config
        let config = Config::load();
        let default_config = config.as_ref().map(|c| &c.default);

        // default mode
        if self.mode.is_none() {
            self.mode = default_config.map(|c| match c.mode.as_str() {
                "timer" => {
                    let timer_config = config.as_ref().map(|c| &c.timer);
                    Mode::Timer {
                        durations: timer_config.map(|c| c.durations.iter().filter_map(|d| parse_duration(d).ok()).collect()).unwrap_or_else(|| vec![Duration::minutes(25), Duration::minutes(5)]),
                        titles: timer_config.map(|c| c.titles.clone()).unwrap_or_default(),
                        repeat: timer_config.map(|c| c.repeat).unwrap_or(false),
                        no_millis: !timer_config.map(|c| c.show_millis).unwrap_or(true),
                        paused: timer_config.map(|c| c.start_paused).unwrap_or(false),
                        auto_quit: timer_config.map(|c| c.auto_quit).unwrap_or(false),
                        execute: timer_config.map(|c| c.execute.clone()).unwrap_or_default(),
                    }
                },
                "stopwatch" => Mode::Stopwatch,
                "countdown" => {
                    let countdown_config = config.as_ref().map(|c| &c.countdown);
                    Mode::Countdown {
                        time: countdown_config.and_then(|c| c.time.as_ref()).and_then(|t| parse_datetime(t).ok()).unwrap_or_else(|| Local::now()),
                        title: countdown_config.map(|c| c.title.clone()).unwrap_or(None),
                        continue_on_zero: countdown_config.map(|c| c.continue_on_zero).unwrap_or(false),
                        reverse: countdown_config.map(|c| c.reverse).unwrap_or(false),
                        millis: countdown_config.map(|c| c.show_millis).unwrap_or(false),
                    }
                },
                _ => {
                    let clock_config = config.as_ref().map(|c| &c.clock);
                    Mode::Clock {
                        no_date: !clock_config.map(|c| c.show_date).unwrap_or(true),
                        millis: clock_config.map(|c| c.show_millis).unwrap_or(false),
                        no_seconds: !clock_config.map(|c| c.show_seconds).unwrap_or(true),
                        timezone: clock_config.and_then(|c| c.timezone),
                    }
                }
            });
        }

        // set default color and size
        if self.color.is_none() {
            self.color = default_config
                .map(|c| parse_color(&c.color).unwrap_or(Color::Green))
                .or(Some(Color::Green));
        }
        if self.size.is_none() {
            self.size = default_config.map(|c| c.size).or(Some(1));
        }

        let style = Style::default().fg(self.color.unwrap_or(Color::Green));
        let size = self.size.unwrap_or(1);

        // initialize the clock mode
        match self.mode.as_ref().unwrap_or(&Mode::Clock {
            no_date: false,
            millis: false,
            no_seconds: false,
            timezone: None,
        }) {
            Mode::Clock {
                no_date,
                no_seconds,
                millis,
                timezone,
            } => {
                let clock_config = config.as_ref().map(|c| &c.clock);
                self.clock = Some(Clock {
                    size,
                    style,
                    show_date: !no_date && clock_config.map(|c| c.show_date).unwrap_or(true),
                    show_millis: *millis || clock_config.map(|c| c.show_millis).unwrap_or(false),
                    show_secs: !no_seconds && clock_config.map(|c| c.show_seconds).unwrap_or(true),
                    timezone: timezone.or_else(|| clock_config.and_then(|c| c.timezone)),
                });
            }
            Mode::Timer {
                durations,
                titles,
                repeat,
                no_millis,
                paused,
                auto_quit,
                execute,
            } => {
                let timer_config = config.as_ref().map(|c| &c.timer);
                let format = if *no_millis {
                    DurationFormat::HourMinSec
                } else {
                    DurationFormat::HourMinSecDeci
                };
                self.timer = Some(Timer::new(
                    size,
                    style,
                    durations.to_owned(),
                    titles.to_owned(),
                    *repeat || timer_config.map(|c| c.repeat).unwrap_or(false),
                    format,
                    *paused || timer_config.map(|c| c.start_paused).unwrap_or(false),
                    *auto_quit || timer_config.map(|c| c.auto_quit).unwrap_or(false),
                    execute.to_owned(),
                ));
            }
            Mode::Stopwatch => {
                self.stopwatch = Some(Stopwatch::new(size, style));
            }
            Mode::Countdown {
                time,
                title,
                continue_on_zero,
                reverse,
                millis,
            } => {
                let countdown_config = config.as_ref().map(|c| &c.countdown);
                self.countdown = Some(Countdown {
                    size,
                    style,
                    time: *time,
                    title: title.to_owned(),
                    continue_on_zero: *continue_on_zero
                        || countdown_config.map(|c| c.continue_on_zero).unwrap_or(false),
                    reverse: *reverse || countdown_config.map(|c| c.reverse).unwrap_or(false),
                    format: if *millis || countdown_config.map(|c| c.show_millis).unwrap_or(false) {
                        DurationFormat::HourMinSecDeci
                    } else {
                        DurationFormat::HourMinSec
                    },
                })
            }
        }
    }

    pub fn ui(&self, f: &mut Frame) {
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

    pub fn is_ended(&self) -> bool {
        if let Some(ref w) = self.timer {
            return w.is_finished();
        }
        false
    }

    pub fn on_exit(&self) {
        if let Some(ref w) = self.stopwatch {
            println!("Stopwatch time: {}", w.get_display_time());
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
    let today = Local::now().date_naive();

    let time = NaiveTime::parse_from_str(s, "%H:%M");
    if let Ok(time) = time {
        let time = NaiveDateTime::new(today, time);
        return Ok(Local.from_local_datetime(&time).unwrap());
    }

    let time = NaiveTime::parse_from_str(s, "%H:%M:%S");
    if let Ok(time) = time {
        let time = NaiveDateTime::new(today, time);
        return Ok(Local.from_local_datetime(&time).unwrap());
    }

    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d");
    if let Ok(date) = date {
        let time = NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        return Ok(Local.from_local_datetime(&time).unwrap());
    }

    let date_time = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S");
    if let Ok(date_time) = date_time {
        return Ok(Local.from_local_datetime(&date_time).unwrap());
    }

    let rfc_time = DateTime::parse_from_rfc3339(s);
    if let Ok(rfc_time) = rfc_time {
        return Ok(rfc_time.with_timezone(&Local));
    }

    Err("Invalid time format".to_string())
}

fn parse_timezone(s: &str) -> Result<Tz, String> {
    s.parse()
}
