use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::process;

use libc;

pub struct Shell {
    pub terminal: i32,
    pub is_interactive: bool,
    pub pgid: i32,
    pub tmodes: Box<libc::termios>,
}

impl Shell {
    pub unsafe fn init() -> Shell {
        #[cfg(target_env = "musl")]
        let termios = libc::termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            __c_ispeed: 0,
            __c_ospeed: 0,
        };

        #[cfg(target_env = "gnu")]
        let termios = libc::termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        };

        let terminal = libc::STDIN_FILENO;
        let is_interactive: bool = libc::isatty(terminal) == 1;
        let tmodes: Box<libc::termios> = Box::new(termios);
        let tmodes_ptr = Box::into_raw(tmodes);

        let mut pgid = libc::getpgrp();
        if is_interactive {
            while libc::tcgetpgrp(terminal) != pgid {
                pgid = libc::getpgrp();
                libc::kill(-pgid, libc::SIGTTIN);
            }

            libc::signal(libc::SIGINT, libc::SIG_IGN);
            libc::signal(libc::SIGQUIT, libc::SIG_IGN);
            libc::signal(libc::SIGTSTP, libc::SIG_IGN);
            libc::signal(libc::SIGTTIN, libc::SIG_IGN);
            libc::signal(libc::SIGTTOU, libc::SIG_IGN);
            libc::signal(libc::SIGCHLD, libc::SIG_IGN);

            // NOTE: I don't think this is actually needed
            //let pid = libc::getpid();
            //if libc::setpgid(pid, pgid) < 0 {
            //    eprintln!("Couldn't put shell in its own process group");
            //    process::exit(1);
            //}

            libc::tcsetpgrp(terminal, pgid);
            libc::tcgetattr(terminal, tmodes_ptr);
        }

        Shell {
            terminal: terminal,
            is_interactive: is_interactive,
            pgid: pgid,
            tmodes: Box::from_raw(tmodes_ptr),
        }
    }

    pub fn launch_process(&mut self, argv: String) -> Result<(), Box<dyn Error>> {
        let mut command = process::Command::new("");
        for (i, arg) in argv.split_whitespace().enumerate() {
            if i == 0 {
                if is_valid_command(arg) {
                    command = process::Command::new(arg);
                } else {
                    println!("{arg}: command not found");
                    return Ok(());
                }
            } else {
                command.arg(arg);
            }
        }
        unsafe {
            if self.is_interactive {
                let pid = libc::getpid();
                if self.pgid == 0 {
                    self.pgid = pid
                };
                libc::setpgid(pid, self.pgid);
                libc::tcsetpgrp(self.terminal, self.pgid);
                let status = command
                    .pre_exec(|| {
                        libc::signal(libc::SIGINT, libc::SIG_DFL);
                        libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                        libc::signal(libc::SIGTSTP, libc::SIG_DFL);
                        libc::signal(libc::SIGTTIN, libc::SIG_DFL);
                        libc::signal(libc::SIGTTOU, libc::SIG_DFL);
                        libc::signal(libc::SIGCHLD, libc::SIG_DFL);
                        Ok(())
                    })
                    .status();
                match status {
                    Ok(_) => (),
                    Err(_) => (),
                }
            } else {
                let status = command.status();
                match status {
                    Ok(_) => (),
                    Err(_) => (),
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{terminal: {}, is_interactive: {}, pgid: {}}}",
            self.terminal, self.is_interactive, self.pgid
        )
    }
}

fn is_valid_command(program: &str) -> bool {
    if let "." | "./" | ".." | "../" = program {
        return false;
    }
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    let meta = fs::metadata(program);
    if meta.is_ok() {
        let mode = meta.unwrap().permissions().mode();
        if (mode & 0o111) != 0 {
            return true;
        }
    }
    false
}
