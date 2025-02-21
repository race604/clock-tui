use chrono::Duration;
use chrono_tz::Tz;
use serde::{Deserialize, Deserializer};
use std::path::PathBuf;

fn deserialize_timezone<'de, D>(deserializer: D) -> Result<Option<Tz>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => s.parse().map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub default: DefaultConfig,
    #[serde(default)]
    pub clock: ClockConfig,
    #[serde(default)]
    pub timer: TimerConfig,
    #[serde(default)]
    pub stopwatch: StopwatchConfig,
    #[serde(default)]
    pub countdown: CountdownConfig,
}

#[derive(Debug, Deserialize)]
pub struct DefaultConfig {
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default = "default_color")]
    pub color: String,
    #[serde(default = "default_size")]
    pub size: u16,
}

#[derive(Debug, Deserialize)]
pub struct ClockConfig {
    #[serde(default = "default_true")]
    pub show_date: bool,
    #[serde(default = "default_true")]
    pub show_seconds: bool,
    #[serde(default = "default_false")]
    pub show_millis: bool,
    #[serde(default, deserialize_with = "deserialize_timezone")]
    pub timezone: Option<Tz>,
}

#[derive(Debug, Deserialize)]
pub struct TimerConfig {
    #[serde(default = "default_timer_durations")]
    pub durations: Vec<String>,
    #[serde(default)]
    pub titles: Vec<String>,
    #[serde(default)]
    pub repeat: bool,
    #[serde(default = "default_true")]
    pub show_millis: bool,
    #[serde(default)]
    pub start_paused: bool,
    #[serde(default)]
    pub auto_quit: bool,
    #[serde(default)]
    pub execute: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct StopwatchConfig {}

#[derive(Debug, Deserialize)]
pub struct CountdownConfig {
    #[serde(default)]
    pub time: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub show_millis: bool,
    #[serde(default)]
    pub continue_on_zero: bool,
    #[serde(default)]
    pub reverse: bool,
}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self {
            mode: default_mode(),
            color: default_color(),
            size: default_size(),
        }
    }
}

impl Default for ClockConfig {
    fn default() -> Self {
        Self {
            show_date: default_true(),
            show_seconds: default_true(),
            show_millis: default_false(),
            timezone: None,
        }
    }
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self {
            durations: default_timer_durations(),
            titles: Vec::new(),
            repeat: false,
            show_millis: default_true(),
            start_paused: false,
            auto_quit: false,
            execute: Vec::new(),
        }
    }
}

impl Default for StopwatchConfig {
    fn default() -> Self {
        Self {}
    }
}

impl Default for CountdownConfig {
    fn default() -> Self {
        Self {
            time: None,
            title: None,
            show_millis: false,
            continue_on_zero: false,
            reverse: false,
        }
    }
}

fn default_mode() -> String {
    "clock".to_string()
}

fn default_color() -> String {
    "green".to_string()
}

fn default_size() -> u16 {
    1
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_timer_durations() -> Vec<String> {
    vec!["25m".to_string(), "5m".to_string()]
}

impl Config {
    pub fn load() -> Option<Self> {
        // ~/.config/tclock/config.toml
        let config_path = dirs::home_dir()?
            .join(".config")
            .join("tclock")
            .join("config.toml");

        if !config_path.exists() {
            return None;
        };

        let content = std::fs::read_to_string(config_path).ok()?;
        match toml::from_str(&content) {
            Ok(config) => Some(config),
            Err(e) => {
                eprintln!("解析配置文件失败: {}", e);
                None
            }
        }
    }
}
