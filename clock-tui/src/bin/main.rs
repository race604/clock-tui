use std::error::Error;
use std::io::{self, Write};
use std::time::Duration;

use clap::Parser;
use clock_tui::app::App;
use clock_tui::app::Mode;
use clock_tui::config::Config;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    // Must be done first so `--help` isn't printed to the alternate screen.
    let mut app = App::parse();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load config and initialize app
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
                    _ => {}
                }
            }
        }
    }

    // restore terminal
    terminal.show_cursor()?;
    drop(terminal);
    disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;

    // Perform logic such as printing the stopwatch time.
    // Must be done after leaving alternate screen.
    app.on_exit();
    io::stdout().flush()?;

    Ok(())
}
