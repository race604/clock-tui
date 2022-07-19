use chrono::Duration;
use crossterm::event::KeyCode;
use regex::Regex;
use tui::backend::Backend;
use tui::style::Color;
use tui::style::Style;
use tui::Frame;

use self::modes::Clock;
use self::modes::Stopwatch;
use self::modes::Timer;

pub(crate) mod modes;

#[derive(clap::ArgEnum, Clone)]
pub(crate) enum Mode {
    Clock,
    Timer,
    Stopwatch,
}

#[derive(clap::Parser)]
pub(crate) struct App {
    #[clap(short, long, value_parser, arg_enum, default_value = "clock")]
    pub mode: Mode,
    #[clap(short, long, value_parser = parse_color, default_value = "green")]
    pub color: Color,
    #[clap(short, long, value_parser = parse_duration, default_value = "5m")]
    pub duration: Duration,
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
        match self.mode {
            Mode::Clock => {
                self.clock = Some(Clock {
                    size: self.size,
                    style,
                    long: false,
                });
            }
            Mode::Timer => {
                self.timer = Some(Timer::new(self.duration, self.size, style));
            }
            Mode::Stopwatch => {
                self.stopwatch = Some(Stopwatch::new(self.size, style));
            }
        }
    }

    pub fn ui<B: Backend>(&self, f: &mut Frame<B>) {
        if let Some(w) = self.clock.as_ref() {
            f.render_widget(w, f.size());
        } else if let Some(w) = self.timer.as_ref() {
            f.render_widget(w, f.size());
        } else if let Some(w) = self.stopwatch.as_ref() {
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
            let r = hex[1..3].parse::<u8>().unwrap();
            let g = hex[3..5].parse::<u8>().unwrap();
            let b = hex[5..7].parse::<u8>().unwrap();
            Ok(Color::Rgb(r, g, b))
        }
    }
}
