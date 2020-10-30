use core::option::Option::Some;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use cursive::Cursive;

use crate::card::card_data::CardSet;
use crate::card::card_ui::display;

pub fn next_card(siv: &mut Cursive) {
    update_card_set(siv, |card_set| card_set.next_card());
}

pub fn prev_card(siv: &mut Cursive) {
    update_card_set(siv, |card_set| card_set.prev_card());
}

pub fn reverse_card(siv: &mut Cursive) {
    update_card_set(siv, |card_set| card_set.reverse_current_card());
}

pub fn toggle_pronunciation(siv: &mut Cursive) {
    update_card_set(siv, |card_set| card_set.toggle_show_pronunciation());
}

pub fn toggle_description(siv: &mut Cursive) {
    update_card_set(siv, |card_set| card_set.toggle_show_description());
}

pub fn toggle_example(siv: &mut Cursive) {
    update_card_set(siv, |card_set| card_set.toggle_show_example());
}

pub fn toggle_title(siv: &mut Cursive) {
    update_card_set(siv, |card_set| card_set.toggle_show_title());
}

pub fn toggle_hints(siv: &mut Cursive) {
    update_card_set(siv, |card_set| card_set.toggle_show_hints());
}

pub fn toggle_zen_mode(siv: &mut Cursive) {
    update_card_set(siv, |card_set| card_set.toggle_zen_mode());
}

pub fn refresh(siv: &mut Cursive) {
    update_card_set(siv, |card_set| {
        card_set;
    });
}

fn update_card_set<F>(siv: &mut Cursive, cb: F)
where
    F: FnOnce(&mut CardSet),
{
    if let Some(card_set) = siv.user_data::<Rc<RefCell<CardSet>>>().cloned() {
        let mut card_set: RefMut<CardSet> = RefCell::borrow_mut(&card_set);
        cb(&mut card_set);
        display(siv, &card_set);
    }
}
