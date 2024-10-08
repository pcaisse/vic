pub mod buffer;
pub mod error;
pub mod mode;

use self::buffer::Buffer;
use self::error::OpError;
use self::mode::Mode;
use crossterm::event::KeyCode;

enum Op {
    EnterCommandMode,
    EnterInsertMode,
    EnterInsertModeAppend,
    EnterNormalMode,
    Quit,
    // TODO: Make these chars into strings to accomodate eg. copy/pasting
    PushToCommand { command: String, char: char },
    PopFromCommand { command: String },
    Insert { char: char },
    MoveBigWordForward,
    MoveBigWordBackward,
}

fn next_op(mode: &Mode, code: KeyCode) -> Result<Op, OpError> {
    match (mode, code) {
        // Enter into command mode
        (Mode::Normal, KeyCode::Char(':')) => Ok(Op::EnterCommandMode),
        // Enter into insert mode
        (Mode::Normal, KeyCode::Char('i')) => Ok(Op::EnterInsertMode),
        (Mode::Normal, KeyCode::Char('a')) => Ok(Op::EnterInsertModeAppend),
        // Add to current command
        (Mode::CommandLine { command }, KeyCode::Char(c)) => Ok(Op::PushToCommand {
            command: command.to_owned(),
            char: c,
        }),
        // Delete from current command
        (Mode::CommandLine { command }, KeyCode::Backspace) => Ok(Op::PopFromCommand {
            command: command.to_owned(),
        }),
        // Quit
        (Mode::CommandLine { command }, KeyCode::Enter) => {
            if command == "q" {
                Ok(Op::Quit)
            } else {
                Err(OpError::InvalidCommandError {
                    command: command.to_owned(),
                })
            }
        }
        // Move forwards one bigword
        (Mode::Normal, KeyCode::Char('W')) => Ok(Op::MoveBigWordForward),
        // Move backwards one bigword
        (Mode::Normal, KeyCode::Char('B')) => Ok(Op::MoveBigWordBackward),
        // Exit insert or command line mode
        (Mode::Insert | Mode::CommandLine { .. }, KeyCode::Esc) => Ok(Op::EnterNormalMode),
        // Insert char
        (Mode::Insert, KeyCode::Char(c)) => Ok(Op::Insert { char: c }),
        (_, code) => Err(OpError::UnknownKeyCodeError { code }),
    }
}

pub struct EditorState {
    pub mode: Mode,
    pub buffer: Buffer,
    // TODO: save buffer to file on :w command
    // TODO: load file contents into buffer when editor loads
    pub quit: bool,
    pub error: Option<OpError>,
}

impl EditorState {
    pub fn update(&mut self, code: KeyCode) -> &mut EditorState {
        match next_op(&self.mode, code) {
            Ok(Op::EnterCommandMode) => {
                self.mode = Mode::CommandLine {
                    command: String::new(),
                };
                self.error = None;
            }
            Ok(Op::EnterInsertModeAppend) => {
                self.mode = Mode::Insert;
                if !self.buffer.text.is_empty()
                    && self.buffer.grapheme_index < self.buffer.text.len()
                {
                    self.buffer.grapheme_index += 1;
                }
                self.error = None;
            }
            Ok(Op::EnterInsertMode) => {
                self.mode = Mode::Insert;
                self.error = None;
            }
            Ok(Op::EnterNormalMode) => {
                self.mode = Mode::Normal;
                self.error = None;
                self.buffer.grapheme_index -= 1;
            }
            Ok(Op::Quit) => self.quit = true,
            Ok(Op::PushToCommand { mut command, char }) => {
                command.push(char);
                self.mode = Mode::CommandLine { command };
                self.error = None;
            }
            Ok(Op::PopFromCommand { mut command }) => {
                command.pop();
                self.mode = Mode::CommandLine { command };
                self.error = None;
            }
            Ok(Op::MoveBigWordForward) => {
                self.buffer.move_big_word_forwards();
            }
            Ok(Op::MoveBigWordBackward) => {
                self.buffer.move_big_word_backwards();
            }
            Ok(Op::Insert { char }) => {
                self.buffer.insert(char);
            }
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
            buffer: Buffer {
                ..Default::default()
            },
            quit: false,
            error: None,
        }
    }
}
