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
}

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut buffer = String::new();
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
                match (mode.clone(), code) {
                    (Mode::Normal, KeyCode::Esc) => {
                        break;
                    }
                    (Mode::Normal, KeyCode::Char('i')) => {
                        mode = Mode::Insert
                    }
                    (Mode::Insert, KeyCode::Esc) => {
                        mode = Mode::Normal
                    }
                    (Mode::Insert, KeyCode::Char(c)) => {
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
