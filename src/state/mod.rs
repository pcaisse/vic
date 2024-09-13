pub mod error;

use self::error::OpError;
use crossterm::event::KeyCode;

#[derive(Clone, Debug, PartialEq)]
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
    PopFromCommand { command: String },
    PushToBuffer { char: char },
}

fn next_op(mode: Mode, code: KeyCode) -> Result<Op, OpError> {
    match (mode.clone(), code) {
        // Enter into command mode
        (Mode::Normal, KeyCode::Char(':')) => Ok(Op::EnterCommandMode),
        // Enter into insert mode
        (Mode::Normal, KeyCode::Char('i')) => Ok(Op::EnterInsertMode),
        // Add to current command
        (Mode::CommandLine { command }, KeyCode::Char(c)) => {
            Ok(Op::PushToCommand { command, char: c })
        }
        // Delete from current command
        (Mode::CommandLine { command }, KeyCode::Backspace) => Ok(Op::PopFromCommand { command }),
        // Quit
        (Mode::CommandLine { command }, KeyCode::Enter) => {
            if command == "q" {
                Ok(Op::Quit)
            } else {
                Err(OpError::InvalidCommandError { command })
            }
        }
        // Exit insert or command line mode
        (Mode::Insert | Mode::CommandLine { .. }, KeyCode::Esc) => Ok(Op::EnterNormalMode),
        // Append to text buffer
        (Mode::Insert, KeyCode::Char(c)) => Ok(Op::PushToBuffer { char: c }),
        (_, code) => Err(OpError::UnknownKeyCodeError { code }),
    }
}

pub struct EditorState {
    pub mode: Mode,
    pub buffer: String,
    pub quit: bool,
    pub error: Option<OpError>,
}

impl EditorState {
    pub fn update(&mut self, code: KeyCode) -> &mut EditorState {
        match next_op(self.mode.clone(), code) {
            Ok(Op::EnterCommandMode) => {
                self.mode = Mode::CommandLine {
                    command: String::new(),
                };
                self.error = None;
            }
            Ok(Op::EnterInsertMode) => {
                self.mode = Mode::Insert;
                self.error = None;
            }
            Ok(Op::EnterNormalMode) => {
                self.mode = Mode::Normal;
                self.error = None;
            }
            Ok(Op::Quit) => self.quit = true,
            Ok(Op::PushToCommand { command, char }) => {
                self.mode = Mode::CommandLine {
                    command: format!("{command}{char}"),
                };
                self.error = None;
            }
            Ok(Op::PopFromCommand { mut command }) => {
                command.pop();
                self.mode = Mode::CommandLine { command };
                self.error = None;
            }
            Ok(Op::PushToBuffer { char }) => self.buffer.push(char),
            Err(error) => {
                self.error = Some(error);
                self.mode = Mode::Normal;
            }
        }
        self
    }
}

impl Default for EditorState {
    fn default() -> EditorState {
        EditorState {
            mode: Mode::Normal,
            buffer: String::new(),
            quit: false,
            error: None,
        }
    }
}
