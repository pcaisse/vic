use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout, Stylize, Terminal},
    widgets::{Block, Borders, Paragraph},
};
use std::io::{stdout, Result};

#[derive(Clone)]
enum Mode {
    Normal,
    Insert,
    CommandLine { command: String },
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

            match mode.clone() {
                Mode::CommandLine { command } => {
                    let layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(vec![Constraint::Percentage(100), Constraint::Min(2)])
                        .split(area);
                    frame.render_widget(
                        Paragraph::new(buffer.clone()).white().on_black(),
                        layout[0],
                    );
                    frame.render_widget(
                        Paragraph::new([":", command.clone().as_str()].join(""))
                            .white()
                            .block(Block::new().borders(Borders::TOP)),
                        layout[1],
                    );
                }
                _ => {
                    frame.render_widget(Paragraph::new(buffer.clone()).white().on_black(), area);
                }
            }
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match (mode.clone(), code) {
                    // Enter into command mode
                    (Mode::Normal, KeyCode::Char(':')) => {
                        mode = Mode::CommandLine {
                            command: String::new(),
                        };
                    }
                    // Enter into insert mode
                    (Mode::Normal, KeyCode::Char('i')) => {
                        mode = Mode::Insert;
                    }
                    // Add to current command
                    (Mode::CommandLine { mut command }, KeyCode::Char(c)) => {
                        command.push(c);
                        mode = Mode::CommandLine { command };
                    }
                    // Quit
                    (Mode::CommandLine { command }, KeyCode::Enter) => {
                        if command == "q" {
                            break;
                        }
                    }
                    // Exit insert or command line mode
                    (Mode::Insert | Mode::CommandLine { .. }, KeyCode::Esc) => {
                        mode = Mode::Normal;
                    }
                    // Append to text buffer
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
