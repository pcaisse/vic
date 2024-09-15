use std::fmt;

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Mode::Normal => String::from("NORMAL"),
            Mode::Insert => String::from("INSERT"),
            Mode::CommandLine { .. } => String::from("COMMAND"),
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    CommandLine { command: String },
}
