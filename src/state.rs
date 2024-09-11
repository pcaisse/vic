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
    PushToBuffer { char: char },
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

pub struct EditorState {
    pub mode: Mode,
    pub buffer: String,
    pub quit: bool,
}

impl EditorState {
    pub fn update(&mut self, code: KeyCode) -> &mut EditorState {
        match next_op(self.mode.clone(), code) {
            Ok(Op::EnterCommandMode) => {
                self.mode = Mode::CommandLine {
                    command: String::new(),
                }
            }
            Ok(Op::EnterInsertMode) => self.mode = Mode::Insert,
            Ok(Op::EnterNormalMode) => self.mode = Mode::Normal,
            Ok(Op::Quit) => self.quit = true,
            Ok(Op::PushToCommand { command, char }) => {
                self.mode = Mode::CommandLine {
                    command: format!("{command}{char}"),
                }
            }
            Ok(Op::PushToBuffer { char }) => self.buffer.push(char),
            Err(_msg) => {
                // TODO: Display error message
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
        }
    }
}
