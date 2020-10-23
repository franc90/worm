use std::cell::RefCell;
use std::rc::Rc;

use cursive::Cursive;
use cursive::traits::*;
use cursive::view::SizeConstraint;
use cursive::views::TextView;

use crate::select::select_data::SelectData;

pub const CARD_VIEW_NAME: &str = "card";

pub fn setup_deck(siv: &mut Cursive) {
    if let Some(app_data) = siv.user_data::<Rc<RefCell<SelectData>>>().cloned() {
        let init_text = app_data
            .borrow()
            .get_selected_set()
            .unwrap()
            .get_text()
            .unwrap()
            .clone();

        let text_view = TextView::new(init_text)
            .center()
            .with_name(CARD_VIEW_NAME)
            .resized(SizeConstraint::Full, SizeConstraint::Full);

        siv.add_fullscreen_layer(text_view);
    }
}
