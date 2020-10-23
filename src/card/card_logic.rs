use core::option::Option::Some;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use cursive::Cursive;
use cursive::views::{TextView, ViewRef};

use crate::select::select_data::SelectData;
use crate::card::card_data::CardSet;
use crate::card::card_ui::CARD_VIEW_NAME;

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
    if let Some(mut view) = siv.find_name::<TextView>(CARD_VIEW_NAME) {
        if let Some(app_data) = siv.user_data::<Rc<RefCell<SelectData>>>().cloned() {
            let mut data: RefMut<SelectData> = RefCell::borrow_mut(&app_data);
            let card_set: &mut CardSet = data.get_selected_set_mut().unwrap();
            cb(card_set);
            update_display(siv, &mut view, &card_set);
        }
    }
}

fn update_display(_: &mut Cursive, view: &mut ViewRef<TextView>, card_set: &CardSet) {
    match card_set.get_text() {
        Some(card_text) => view.set_content(card_text),
        None => (),
    }
}
