use rustyline::Editor;
use std::error;

use crab_shell;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut shell = unsafe { crab_shell::Shell::init() };
    let mut rl = Editor::<()>::new()?;
    loop {
        let readline = rl.readline("$ ");

        match readline {
            Ok(line) => {
                match line.trim_end() {
                    "quit" | "exit" => return Ok(()),
                    "" => {}
                    "history" => {
                        for (i, h) in rl.history().iter().enumerate() {
                            println!("{}: {}", i, h);
                        }
                    }
                    _ => shell.launch_process(line.clone())?,
                }
                rl.add_history_entry(line);
            }
            Err(_) => {}
        }
    }
}
