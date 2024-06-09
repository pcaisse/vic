use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

#[derive(Copy, Clone)]
enum Mode {
    Normal,
    Insert,
    CommandLine
}

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut buffer = String::new();
    let mut command = String::new();
    let mut mode = Mode::Normal;

    loop {
        // Draw text buffer
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new(buffer.clone())
                    .white()
                    .on_black(),
                area,
            );
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match (mode.clone(), code, command.as_str()) {
                    // Enter into command mode
                    (Mode::Normal, KeyCode::Char(':'), _) => {
                        command = String::new();
                        mode = Mode::CommandLine;
                    }
                    // Enter into insert mode
                    (Mode::Normal, KeyCode::Char('i'), _) => {
                        mode = Mode::Insert;
                    }
                    // Add to current command
                    (Mode::CommandLine, KeyCode::Char(c), _) => {
                        command.push(c);
                    }
                    // Quit
                    (Mode::CommandLine, KeyCode::Enter, "q") => {
                        break;
                    }
                    // Exit insert or command line mode
                    (Mode::Insert | Mode::CommandLine, KeyCode::Esc, _) => {
                        mode = Mode::Normal;
                    }
                    // Append to text buffer
                    (Mode::Insert, KeyCode::Char(c), _) => {
                        buffer.push(c);
                    }
                    _ => {}
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
