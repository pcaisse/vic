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

pub mod state;
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

            let mut render_layout_with_text = |text: Paragraph| {
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
                frame.render_widget(text.block(Block::new().borders(Borders::TOP)), layout[1]);
            };

            match (editor_state.mode.clone(), editor_state.error.clone()) {
                (_, Some(error)) => {
                    let error_text = Paragraph::new(error.to_string()).red();
                    render_layout_with_text(error_text)
                }
                (Mode::CommandLine { command }, None) => {
                    let command_text =
                        Paragraph::new([":", command.clone().as_str()].join("")).white();
                    render_layout_with_text(command_text);
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
