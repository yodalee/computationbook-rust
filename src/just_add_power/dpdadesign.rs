
use super::pdaconfiguration::{PDAConfiguration};
use super::dpda::{DPDA};
use super::dpdarulebook::{DPDARulebook};

pub struct DPDADesign {
    start_state: u32,
    bottom_char: char,
    accept_state: Vec<u32>,
    rulebook: DPDARulebook,
}

impl DPDADesign {
    pub fn new(start_state: u32, bottom_char: char, accept_state: &[u32], rulebook: &DPDARulebook) -> Self {
        DPDADesign{
            start_state: start_state,
            bottom_char: bottom_char,
            accept_state: accept_state.to_vec(),
            rulebook: rulebook.clone(),
        }
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut to_dpda = DPDA::new(
            &PDAConfiguration::new(self.start_state, &[self.bottom_char]),
            &self.accept_state, &self.rulebook
        );
        to_dpda.read_string(s);
        to_dpda.accept()
    }
}
