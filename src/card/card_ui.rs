use std::cell::RefCell;
use std::rc::Rc;

use cursive::Cursive;
use cursive::traits::*;
use cursive::view::SizeConstraint;
use cursive::views::{LinearLayout, TextView};

use crate::card::card_data::CardSet;

pub const CARD_VIEW_NAME: &str = "card";

pub fn setup_card_view(siv: &mut Cursive, main_view: &mut LinearLayout) {
    main_view.insert_child(
        0,
        if let Some(card_set) = siv.user_data::<Rc<RefCell<CardSet>>>().cloned() {
            let card_set = card_set.borrow();
            let init_text = card_set.get_text().unwrap().clone();

            TextView::new(init_text)
                .center()
                .with_name(CARD_VIEW_NAME)
                .resized(SizeConstraint::Full, SizeConstraint::Full)
        } else {
            TextView::new("No card set selected")
                .center()
                .with_name(CARD_VIEW_NAME)
                .resized(SizeConstraint::Full, SizeConstraint::Full)
        },
    );
}
