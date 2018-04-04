use super::pdaconfiguration::{PDAConfiguration};
use super::npda::{NPDA};
use super::npdarulebook::{NPDARulebook};

use helper::{to_hashset};

pub struct NPDADesign {
    start_state: u32,
    bottom_char: char,
    accept_state: Vec<u32>,
    rulebook: NPDARulebook,
}

impl NPDADesign {
    pub fn new(start_state: u32, bottom_char: char, accept_state: &[u32], rulebook: &NPDARulebook) -> Self {
        NPDADesign{
            start_state: start_state,
            bottom_char: bottom_char,
            accept_state: accept_state.to_vec(),
            rulebook: rulebook.clone(),
        }
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut to_npda = NPDA::new(
            &to_hashset(&[PDAConfiguration::new(self.start_state, &[self.bottom_char])]),
            &to_hashset(&self.accept_state),
            &self.rulebook
        );
        to_npda.read_string(s);
        to_npda.accept()
    }
}
