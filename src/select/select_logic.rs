use cursive::Cursive;
use cursive::views::{SelectView, TextView};

use crate::card::card_ui;
use crate::card::card_ui::CARD_VIEW_NAME;
use crate::select::select_ui::SELECT_VIEW_NAME;

pub fn go_up(siv: &mut Cursive) {
    if let Some(mut view) = siv.find_name::<SelectView<usize>>(SELECT_VIEW_NAME) {
        let cb = view.select_up(1);
        cb(siv);
    }
}

pub fn go_down(siv: &mut Cursive) {
    if let Some(mut view) = siv.find_name::<SelectView<usize>>(SELECT_VIEW_NAME) {
        let cb = view.select_down(1);
        cb(siv);
    }
}

pub fn show_card_set(siv: &mut Cursive) {
    if let Some(_) = siv.find_name::<SelectView<usize>>(SELECT_VIEW_NAME) {
        if siv.find_name::<TextView>(CARD_VIEW_NAME).is_none() {
            card_ui::setup_deck(siv)
        }
    }
}
