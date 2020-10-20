
use crate::card::card_data::CardSetStruct;
use cursive::Cursive;
use cursive::views::TextView;
use cursive::view::SizeConstraint;
use cursive::traits::*;

use crate::card_logic;

pub fn setup_deck(s: &mut Cursive, card_set: CardSetStruct) {
    let init_text = card_set.borrow().get().unwrap().clone();
    let reverse_card_set = card_set.clone();
    let next_term_card_set = card_set.clone();
    let prev_term_card_set = card_set.clone();

    s.add_global_callback(' ', move |s| card_logic::reverse_card(s, reverse_card_set.clone()));
    s.add_global_callback('n', move |s| card_logic::next_card(s, next_term_card_set.clone()));
    s.add_global_callback('p', move |s| card_logic::prev_card(s, prev_term_card_set.clone()));

    s.pop_layer();
    s.add_fullscreen_layer(
        TextView::new(init_text)
            .center()
            .with_name("card")
            .resized(SizeConstraint::Full, SizeConstraint::Full),
    );
}