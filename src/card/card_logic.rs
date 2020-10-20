use core::option::Option::Some;

use cursive::Cursive;
use cursive::views::TextView;

use crate::card::card_data::CardSetStruct;

pub fn reverse_card(s: &mut Cursive, card_set: CardSetStruct) {
    card_set.borrow_mut().reverse_current_card();
    update_display(s, card_set.clone())
}

pub fn next_card(s: &mut Cursive, card_set: CardSetStruct) {
    card_set.borrow_mut().next_card();
    update_display(s, card_set.clone())
}

pub fn prev_card(s: &mut Cursive, card_set: CardSetStruct) {
    card_set.borrow_mut().prev_card();
    update_display(s, card_set.clone())
}

fn update_display(s: &mut Cursive, card_set: CardSetStruct) {
    let mut view = s.find_name::<TextView>("card").unwrap();
    match card_set.borrow().get() {
        Some(val) => view.set_content(val),
        None => (),
    }
}
