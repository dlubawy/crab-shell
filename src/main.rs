use std::error;
use std::io::{self, Write};

use crab_shell;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut shell = unsafe { crab_shell::Shell::init() };
    let stdin = io::stdin();
    loop {
        let mut buffer = String::new();

        print!("\r$ ");
        io::stdout().flush().unwrap();

        stdin.read_line(&mut buffer)?;
        buffer = buffer.trim_end().to_string();

        match buffer.as_str() {
            "quit" | "exit" => return Ok(()),
            "" => {}
            _ => shell.launch_process(buffer)?,
        }
    }
}
