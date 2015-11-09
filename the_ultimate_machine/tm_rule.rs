use tm_configuration::{TMConfiguration};

#[allow(dead_code)]
pub enum Direction {
    Left,
    Right,
}

pub struct TMRule {
    state: i32,
    character: char,
    next_state: i32,
    write_character: char,
    direction: Direction,
}

impl TMRule {
    pub fn new(state: i32, character: char, next_state: i32, write_character: char, direction: Direction) -> TMRule {
        TMRule {
            state: state,
            character: character,
            next_state: next_state,
            write_character: write_character,
            direction: direction,
        }
    }

    pub fn applies_to(&self, config: &TMConfiguration) -> bool {
        self.state == config.state && self.character == config.tape.middle
    }

    pub fn follow(&self, config: &mut TMConfiguration) {
        config.state = self.next_state;
        self.next_tape(config);
    }

    pub fn next_tape(&self, config: &mut TMConfiguration) {
        config.tape.write(self.write_character);
        match self.direction {
            Direction::Left => config.tape.move_head_left(),
            Direction::Right => config.tape.move_head_right(),
        }
    }
}


