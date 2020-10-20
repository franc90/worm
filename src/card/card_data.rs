use std::cell::RefCell;
use std::rc::Rc;
use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CardData {
    word: String,
    translated: String,
    explanation: String,
    pronunciation: String,
    sentence: String,
    sentence_gap: String,
    gap_term: String,
}
pub type CardSetStruct = Rc<RefCell<CardSet>>;

pub struct CardSet {
    cards: Vec<CardData>,
    current: usize,
    reversed: bool,
}

impl CardSet {
    pub fn new(terms: Vec<CardData>) -> Self {
        Self {
            cards: terms,
            current: 0,
            reversed: false,
        }
    }

    pub fn get(&self) -> Option<String> {
        self.cards.get(self.current).map(|t| {
            if self.reversed {
                t.translated.clone()
            } else {
                t.word.clone()
            }
        })
    }

    pub fn reverse_current_card(&mut self) {
        self.reversed = !self.reversed
    }

    pub fn next_card(&mut self) {
        self.current = if self.current + 1 < self.cards.len() {
            self.current + 1
        } else {
            self.current
        };
    }

    pub fn prev_card(&mut self) {
        self.current = if self.current > 0 { self.current - 1 } else { self.current };
    }
}

