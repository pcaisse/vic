use crossterm::event::KeyCode;
use std::fmt;

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpError::InvalidCommandError { command } => {
                let msg = format!("Invalid command: {command}");
                write!(f, "{msg}")
            }
            OpError::UnknownKeyCodeError { code: _code } => {
                let msg = "Unknown key code";
                write!(f, "{msg}")
            }
        }
    }
}

#[derive(Clone)]
pub enum OpError {
    InvalidCommandError { command: String },
    UnknownKeyCodeError { code: KeyCode },
}
