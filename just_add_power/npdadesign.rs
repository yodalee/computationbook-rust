
use helper::{toHashSet};
use pdaconfiguration::{PDAConfiguration};
use npda::{NPDA};
use npdarulebook::{NPDARulebook};

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
            &toHashSet(&[PDAConfiguration::new(self.start_state, &[self.bottom_char])]),
            &toHashSet(&self.accept_state),
            &self.rulebook
        );
        to_npda.read_string(s);
        to_npda.accept()
    }
}
