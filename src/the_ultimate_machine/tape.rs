use std::fmt;

pub struct Tape {
    left: String,
    pub middle: char,
    right: String,
    blank: char,
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#<Tape {}({}){}>",
            self.left,
            self.middle,
            self.right
        )
    }
}

impl Tape {
    pub fn new(left: &str, middle: char, right: &str, blank: char) -> Tape {
        Tape {
            left: left.to_string(),
            middle: middle,
            right: right.to_string(),
            blank: blank,
        }
    }

    pub fn write(&mut self, c: char) {
        self.middle = c;
    }

    pub fn move_head_left(&mut self) {
        self.right.insert(0, self.middle);
        self.middle = match self.left.pop() {
            Some(c) => c,
            None => self.blank,
        };
    }

    pub fn move_head_right(&mut self) {
        self.left.push(self.middle);
        self.middle = if self.right.len() != 0 {
            self.right.remove(0)
        } else {
            self.blank
        }
    }
}
