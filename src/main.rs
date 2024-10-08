use crossterm::{
    cursor::SetCursorStyle,
    event::{self, Event, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout, Stylize, Terminal, Text},
    widgets::{Block, Borders, Paragraph},
};
use std::io::{stdout, Result as ResultIO};

pub mod state;
use crate::state::mode::Mode;
use crate::state::EditorState;

fn main() -> ResultIO<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut editor_state = EditorState {
        ..Default::default()
    };

    loop {
        let cursor_style = match &editor_state.mode {
            Mode::Insert => SetCursorStyle::SteadyBar,
            _ => SetCursorStyle::DefaultUserShape,
        };
        stdout().execute(cursor_style)?;
        // Draw text buffer
        terminal.draw(|frame| {
            let area = frame.size();

            let msg_text = match (&editor_state.mode, &editor_state.error) {
                (_, Some(error)) => Paragraph::new(error.to_string()).red(),
                (Mode::CommandLine { command }, None) => {
                    Paragraph::new([":", command].join("")).white()
                }
                _ => Paragraph::new(String::new()).white(),
            };

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(100), Constraint::Min(2)])
                .split(area);
            let main_text = Paragraph::new(Text::raw(&editor_state.buffer.text))
                .white()
                .on_black();
            frame.render_widget(main_text, layout[0]);
            frame.render_widget(
                msg_text.block(
                    Block::new()
                        .borders(Borders::TOP)
                        .title(editor_state.mode.to_string()),
                ),
                layout[1],
            );

            let cursor_index = u16::try_from(editor_state.buffer.grapheme_index).unwrap();
            frame.set_cursor(cursor_index % area.width, cursor_index / area.width);
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
