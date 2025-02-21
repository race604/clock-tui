use std::error::Error;
use std::io;
use std::time::Duration;

use clock_tui::app::App;
use clock_tui::config::Config;
use clock_tui::app::Mode;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let config = Config::load().ok_or("Failed to load config")?;
    let mut app = App::default();
    app.init_app();

    loop {
        terminal.draw(|f| app.ui(f))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => app.on_key(KeyCode::Char(' ')),
                    KeyCode::Char('c') => app.set_mode(Mode::Clock {
                        timezone: None,
                        no_date: false,
                        no_seconds: false,
                        millis: false,
                    }),
                    KeyCode::Char('w') => app.set_mode(Mode::Stopwatch),
                    KeyCode::Char('t') => app.set_mode(Mode::Timer {
                        durations: vec![],
                        titles: vec![],
                        repeat: false,
                        no_millis: false,
                        paused: false,
                        auto_quit: false,
                        execute: vec![],
                    }),
                    KeyCode::Char('d') => app.set_mode(Mode::Countdown {
                        time: chrono::Local::now(),
                        title: None,
                        continue_on_zero: false,
                        reverse: false,
                        millis: false,
                    }),
                    _ => {}
                }
            }
        }

        if app.is_ended() {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    app.on_exit();
    Ok(())
}
