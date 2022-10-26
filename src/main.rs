#[macro_use]
extern crate lalrpop_util;

use rustyline::Editor;
use std::error;

use crab_shell;

lalrpop_mod!(pub parser);

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut shell = unsafe { crab_shell::Shell::init() };
    let mut rl = Editor::<()>::new()?;
    let statement_parser = parser::StatementParser::new();
    loop {
        let readline = rl.readline("$ ");

        match readline {
            Ok(line) => {
                let mut stmt = statement_parser.parse(line.as_str()).unwrap();
                loop {
                    let cmd = stmt.pop();
                    match cmd {
                        Some((c, bg)) => match c.first.as_str() {
                            "quit" | "exit" => return Ok(()),
                            "" => {}
                            "history" => {
                                for (i, h) in rl.history().iter().enumerate() {
                                    println!("{}: {}", i, h);
                                }
                            }
                            _ => {
                                shell.launch_process(c, bg)?;
                                //println!("first: {}, args: {}", c.first, c.args.join(" "));
                            }
                        },
                        None => break,
                    }
                }

                rl.add_history_entry(line);
            }
            Err(_) => {}
        }
    }
}
