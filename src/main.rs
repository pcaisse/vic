use crossterm::{
    event::{self, Event, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout, Stylize, Terminal},
    widgets::{Block, Borders, Paragraph},
};
use std::io::{stdout, Result as ResultIO};

mod state;
use crate::state::{EditorState, Mode};

fn main() -> ResultIO<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut editor_state = EditorState {
        ..Default::default()
    };

    loop {
        // Draw text buffer
        terminal.draw(|frame| {
            let area = frame.size();

            match editor_state.mode.clone() {
                Mode::CommandLine { command } => {
                    let layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(vec![Constraint::Percentage(100), Constraint::Min(2)])
                        .split(area);
                    frame.render_widget(
                        Paragraph::new(editor_state.buffer.clone())
                            .white()
                            .on_black(),
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
                    frame.render_widget(
                        Paragraph::new(editor_state.buffer.clone())
                            .white()
                            .on_black(),
                        area,
                    );
                }
            }
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                editor_state.update(code);
                if editor_state.quit {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
