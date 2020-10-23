use crate::card::card_data::CardSet;

#[derive(Debug)]
pub struct SelectData {
    card_sets: Vec<CardSet>,
    pub selected_set: usize,
}

impl SelectData {
    pub fn new(card_sets: Vec<CardSet>) -> Self {
        Self {
            card_sets,
            selected_set: 0,
        }
    }

    pub fn get_selected_set(&self) -> Option<&CardSet> {
        self.card_sets.get(self.selected_set)
    }

    pub fn get_selected_set_mut(&mut self) -> Option<&mut CardSet> {
        self.card_sets.get_mut(self.selected_set)
    }

    pub fn get_sets_name_and_idx(&self) -> Vec<(String, usize)> {
        self.card_sets
            .iter()
            .enumerate()
            .map(|(idx, set)| (set.name.clone(), idx))
            .collect()
    }
}
