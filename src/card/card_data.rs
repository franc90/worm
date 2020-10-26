use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CardData {
    pub word: String,
    pub translated: String,
    pub explanation: String,
    pub pronunciation: String,
    pub sentence: String,
}

#[derive(Debug)]
pub struct CardSet {
    pub name: String,
    cards: Vec<CardData>,
    current_card: usize,
    pub reversed: bool,
}

impl CardSet {
    pub fn new(name: &str, cards: Vec<CardData>) -> Self {
        Self {
            name: name.to_string(),
            cards,
            current_card: 0,
            reversed: false,
        }
    }

    pub fn get_current_card(&self) -> Option<&CardData> {
        self.cards.get(self.current_card)
    }

    pub fn reverse_current_card(&mut self) {
        self.reversed = !self.reversed
    }

    pub fn next_card(&mut self) {
        self.current_card = if self.current_card + 1 < self.cards.len() {
            self.current_card + 1
        } else {
            self.current_card
        };
    }

    pub fn prev_card(&mut self) {
        self.current_card = if self.current_card > 0 {
            self.current_card - 1
        } else {
            self.current_card
        };
    }
}
