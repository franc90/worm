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
    pub current_card: usize,
    reversed: bool,
    show_pronunciation: bool,
    show_description: bool,
    show_example: bool,
    show_title: bool,
}

impl CardSet {
    pub fn new(name: &str, cards: Vec<CardData>) -> Self {
        Self {
            name: name.to_string(),
            cards,
            current_card: 0,
            reversed: false,
            show_pronunciation: true,
            show_description: false,
            show_example: false,
            show_title: true,
        }
    }

    pub fn cards_len(&self) -> usize {
        self.cards.len()
    }

    pub fn toggle_show_pronunciation(&mut self) {
        self.show_pronunciation = !self.show_pronunciation;
    }

    pub fn toggle_show_description(&mut self) {
        self.show_description = !self.show_description;
    }

    pub fn toggle_show_example(&mut self) {
        self.show_example = !self.show_example;
    }

    pub fn toggle_show_title(&mut self) {
        self.show_title = !self.show_title;
    }

    pub fn get_main_text(&self) -> &str {
        let card = self.get_current_card().unwrap();
        if self.reversed {
            &card.translated
        } else {
            &card.word
        }
    }

    pub fn get_title(&self) -> Option<&str> {
        if self.show_title {
            Some(&self.name)
        } else {
            None
        }
    }

    pub fn get_pronunciation(&self) -> Option<&str> {
        let card = self.get_current_card().unwrap();
        if self.show_pronunciation && !self.reversed {
            Some(&card.pronunciation)
        } else {
            None
        }
    }

    pub fn get_desc(&self) -> Option<&str> {
        let card = self.get_current_card().unwrap();
        if self.show_description && !self.reversed {
            Some(&card.explanation)
        } else {
            None
        }
    }

    pub fn get_example(&self) -> Option<&str> {
        let card = self.get_current_card().unwrap();
        if self.show_example && !self.reversed {
            Some(&card.sentence)
        } else {
            None
        }
    }

    fn get_current_card(&self) -> Option<&CardData> {
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
