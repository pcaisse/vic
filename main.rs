use std::io;
use std::io::{Write, stdout};
use crossterm::style::Print;
use crossterm::QueueableCommand;

fn main() -> io::Result<()> {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut stdout = stdout();
        stdout.queue(Print(input))?;
        stdout.flush()?;
    }
}
