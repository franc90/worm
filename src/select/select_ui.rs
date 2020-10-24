use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use cursive::align::HAlign;
use cursive::Cursive;
use cursive::traits::*;
use cursive::view::SizeConstraint;
use cursive::views::{DummyView, LinearLayout, SelectView, TextView};

use crate::select::select_data::SelectData;
use crate::select::select_logic;

pub const SELECT_VIEW_NAME: &str = "select_card_set";

pub fn setup_view(siv: &mut Cursive) {
    if let Some(select_data) = siv.user_data::<Rc<RefCell<SelectData>>>().cloned() {
        let items: Vec<(String, usize)> = select_data.borrow().get_sets_name_and_idx();

        let select_view = SelectView::new()
            .with_all(items)
            .h_align(HAlign::Center)
            .on_submit(|siv, _| select_logic::show_card_set(siv))
            .on_select(update_selected_set)
            .with_name(SELECT_VIEW_NAME)
            .resized(SizeConstraint::Full, SizeConstraint::Full);

        siv.add_fullscreen_layer(
            LinearLayout::vertical()
                .child(TextView::new("Select card set to learn:"))
                .child(DummyView)
                .child(select_view)
                .resized(SizeConstraint::Full, SizeConstraint::Full),
        );
    }
}

fn update_selected_set(siv: &mut Cursive, item: &usize) {
    if let Some(app_data) = siv.user_data::<Rc<RefCell<SelectData>>>().cloned() {
        let mut data: RefMut<SelectData> = RefCell::borrow_mut(&app_data);
        data.selected_set = *item;
    }
}
