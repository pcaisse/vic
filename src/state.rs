use crossterm::event::KeyCode;

#[derive(Clone)]
pub enum Mode {
    Normal,
    Insert,
    CommandLine { command: String },
}

enum Op {
    EnterCommandMode,
    EnterInsertMode,
    EnterNormalMode,
    Quit,
    // TODO: Make these chars into strings to accomodate eg. copy/pasting
    PushToCommand { command: String, char: char },
    PushToBuffer { char: char },
}

enum Signal {
    Exit,
}

fn next_op(mode: Mode, code: KeyCode) -> Result<Op, &'static str> {
    match (mode.clone(), code) {
        // Enter into command mode
        (Mode::Normal, KeyCode::Char(':')) => Ok(Op::EnterCommandMode),
        // Enter into insert mode
        (Mode::Normal, KeyCode::Char('i')) => Ok(Op::EnterInsertMode),
        // Add to current command
        (Mode::CommandLine { command }, KeyCode::Char(c)) => {
            Ok(Op::PushToCommand { command, char: c })
        }
        // Quit
        (Mode::CommandLine { command }, KeyCode::Enter) => {
            if command == "q" {
                Ok(Op::Quit)
            } else {
                Err("unknown command")
            }
        }
        // Exit insert or command line mode
        (Mode::Insert | Mode::CommandLine { .. }, KeyCode::Esc) => Ok(Op::EnterNormalMode),
        // Append to text buffer
        (Mode::Insert, KeyCode::Char(c)) => Ok(Op::PushToBuffer { char: c }),
        _ => Err("unknown op/key code"),
    }
}

fn eval_op(
    op: Result<Op, &'static str>,
    buffer: String,
) -> Result<(Option<Mode>, Option<String>, Option<Signal>), &'static str> {
    // TODO: Make return type be changes only? (mode changes, buffer changes, new signals)
    match op {
        Ok(Op::EnterCommandMode) => Ok((
            Some(Mode::CommandLine {
                command: String::new(),
            }),
            None,
            None,
        )),
        Ok(Op::EnterInsertMode) => Ok((Some(Mode::Insert), None, None)),
        Ok(Op::EnterNormalMode) => Ok((Some(Mode::Normal), None, None)),
        Ok(Op::Quit) => Ok((None, None, Some(Signal::Exit))),
        Ok(Op::PushToCommand { command, char }) => Ok((
            Some(Mode::CommandLine {
                command: format!("{command}{char}"),
            }),
            None,
            None,
        )),
        Ok(Op::PushToBuffer { char }) => Ok((None, Some(format!("{buffer}{char}")), None)),
        Err(msg) => Err(msg),
    }
}

pub struct EditorState {
    pub mode: Mode,
    pub buffer: String,
    pub quit: bool,
}

impl EditorState {
    pub fn update(&mut self, code: KeyCode) -> &mut EditorState {
        let op = next_op(self.mode.clone(), code);
        match eval_op(op, self.buffer.clone()) {
            Ok((Some(new_mode), Some(new_buffer), None)) => {
                self.mode = new_mode;
                self.buffer = new_buffer;
            }
            Ok((Some(new_mode), None, None)) => {
                self.mode = new_mode;
            }
            Ok((None, Some(new_buffer), None)) => {
                self.buffer = new_buffer;
            }
            Ok((_, _, Some(Signal::Exit))) => {
                self.quit = true;
            }
            Ok(_) => {
                // TODO: Figure out if this should be possible
            }
            Err(_msg) => {
                // TODO: Display error message
            }
        }
        self
    }
}
