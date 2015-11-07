use tm_configuration::{TMConfiguration};

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

    pub fn applies_to(&self, config: TMConfiguration) -> bool {
        self.state == config.State && self.character == config.Tape.middle
    }

    pub fn next_tape(&self, mut config: TMConfiguration) {
        config.Tape.write(self.write_character);
        match self.direction {
            Direction::Left => config.Tape.move_head_left(),
            Direction::Right => config.Tape.move_head_right(),
        }
    }
}


