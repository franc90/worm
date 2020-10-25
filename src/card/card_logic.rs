use core::option::Option::Some;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use cursive::Cursive;

use crate::card::card_data::CardSet;
use crate::card::card_ui::update_card_view;

pub fn reverse_card(siv: &mut Cursive) {
    update_card_set(siv, |card_set| card_set.reverse_current_card());
}

pub fn next_card(siv: &mut Cursive) {
    update_card_set(siv, |card_set| card_set.next_card());
}

pub fn prev_card(siv: &mut Cursive) {
    update_card_set(siv, |card_set| card_set.prev_card());
}

fn update_card_set<F>(siv: &mut Cursive, cb: F)
where
    F: FnOnce(&mut CardSet),
{
    if let Some(card_set) = siv.user_data::<Rc<RefCell<CardSet>>>().cloned() {
        let mut card_set: RefMut<CardSet> = RefCell::borrow_mut(&card_set);
        cb(&mut card_set);
        update_card_view(siv, &card_set);
    }
}
