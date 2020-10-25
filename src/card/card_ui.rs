use cursive::align::VAlign;
use cursive::traits::*;
use cursive::view::SizeConstraint;
use cursive::views::{LinearLayout, TextView};
use cursive::Cursive;
use serde::export::Option::Some;

use crate::card::card_data::CardSet;
use crate::layout::MAIN_LAYOUT_NAME;

const CARD_LAYOUT_NAME: &str = "card_layout";
const MAIN_ROW: &str = "main_row";
const ROW_2: &str = "row_2";
const ROW_3: &str = "row_3";
const ROW_4: &str = "row_4";

#[derive(Debug)]
struct CardDisplay {
    main_row: String,
    row2: Option<String>,
    row3: Option<String>,
    row4: Option<String>,
}

impl CardDisplay {
    fn new(main_row: &str) -> Self {
        Self {
            main_row: String::from(main_row),
            row2: None,
            row3: None,
            row4: None,
        }
    }
    fn new_full(main_row: &str, add1: &str, add2: &str, add3: &str) -> Self {
        Self {
            main_row: String::from(main_row),
            row2: Some(String::from(add1)),
            row3: Some(String::from(add2)),
            row4: Some(String::from(add3)),
        }
    }
}

pub fn generate_card_view(siv: &mut Cursive, card_set: &CardSet) {
    if let Some(ref mut main_view) = siv.find_name::<LinearLayout>(MAIN_LAYOUT_NAME) {
        main_view.insert_child(
            0,
            compose_card_layout()
                .with_name(CARD_LAYOUT_NAME)
                .resized(SizeConstraint::Full, SizeConstraint::Full),
        );
    }
    update_card_view(siv, card_set);
}

pub fn update_card_view(siv: &mut Cursive, card_set: &CardSet) {
    let card_display = &convert_to_card_display(card_set);
    if let Some(ref mut main_row) = siv.find_name::<TextView>(MAIN_ROW) {
        main_row.set_content(card_display.main_row.clone());
    }
    if let Some(ref mut row2) = siv.find_name::<TextView>(ROW_2) {
        row2.set_content(match &card_display.row2 {
            Some(txt) => txt,
            None => "",
        });
    }
    if let Some(ref mut row3) = siv.find_name::<TextView>(ROW_3) {
        row3.set_content(match &card_display.row3 {
            Some(txt) => txt,
            None => "",
        });
    }
    if let Some(ref mut row4) = siv.find_name::<TextView>(ROW_4) {
        row4.set_content(match &card_display.row4 {
            Some(txt) => txt,
            None => "",
        });
    }
}

fn convert_to_card_display(card_set: &CardSet) -> CardDisplay {
    let card = card_set.get_current_card().unwrap();
    if card_set.reversed {
        CardDisplay::new(&card.translated)
    } else {
        CardDisplay::new_full(
            &card.word,
            &card.pronunciation,
            &card.explanation,
            &card.sentence,
        )
    }
}

fn compose_card_layout() -> LinearLayout {
    LinearLayout::vertical()
        .child(
            TextView::new("")
                .center()
                .v_align(VAlign::Bottom)
                .with_name(MAIN_ROW)
                .resized(SizeConstraint::Full, SizeConstraint::Full),
        )
        .child(TextView::new(" ").max_height(1))
        .child(TextView::new(" ").center().with_name(ROW_2).max_height(2))
        .child(TextView::new(" ").max_height(1))
        .child(TextView::new(" ").center().with_name(ROW_3).max_height(2))
        .child(TextView::new(" ").max_height(1))
        .child(TextView::new(" ").center().with_name(ROW_4).max_height(2))
        .child(TextView::new(" ").resized(SizeConstraint::Full, SizeConstraint::Full))
}
