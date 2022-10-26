use std::fmt;

pub enum Line {
    End,
    Bg,
}

pub struct Stmt {
    cmds: Vec<(Option<Cmd>, bool)>,
}

impl Stmt {
    pub fn new(c: Cmd) -> Self {
        if c.first.len() == 0 {
            Stmt {
                cmds: vec![(None, false)],
            }
        } else {
            Stmt {
                cmds: vec![(Some(c), false)],
            }
        }
    }

    pub fn new_cmd(&mut self, c: Cmd, l: Line) {
        match l {
            Line::End => self.cmds.push((Some(c), false)),
            Line::Bg => self.cmds.push((Some(c), true)),
        }
    }

    pub fn pop(&mut self) -> Option<(Cmd, bool)> {
        match self.cmds.pop() {
            Some((cmd, bg)) => match cmd {
                Some(c) => Some((c, bg)),
                None => None,
            },
            None => None,
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("");
        for (cmd, bg) in self.cmds.iter() {
            match cmd {
                Some(c) => {
                    if s == "" {
                        s = format!("(cmd: {}, bg: {})", c, bg);
                    } else {
                        s = format!("(cmd: {}, bg: {}), {}", c, bg, s);
                    }
                }
                None => {}
            }
        }
        write!(f, "{}", s)
    }
}

pub struct Cmd {
    pub first: String,
    pub args: Vec<String>,
}

impl Cmd {
    pub fn new(s: String) -> Self {
        Cmd {
            first: s,
            args: vec![],
        }
    }

    pub fn push(&mut self, s: String) {
        self.args.push(self.first.clone());
        self.first = s.clone();
    }
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = self.first.clone();
        for arg in self.args.iter() {
            s = format!("{} {}", arg, s);
        }
        write!(f, "{}", s)
    }
}
