use cursive::align::HAlign;
use cursive::align::VAlign;
use cursive::Cursive;
use cursive::theme::Effect;
use cursive::traits::*;
use cursive::view::SizeConstraint;
use cursive::views::{LinearLayout, TextView};

use crate::card::card_data::CardSet;

const CARD_LAYOUT_NAME: &str = "card_layout";
const MAIN_ROW: &str = "main_row";
const ROW_2: &str = "row_2";
const ROW_3: &str = "row_3";
const ROW_4: &str = "row_4";
const TITLE: &str = "title";
const SHORTCUTS: &str = "shortcuts";

pub fn generate_card_view(siv: &mut Cursive, card_set: &CardSet) {
    siv.add_fullscreen_layer(
        compose_card_layout()
            .with_name(CARD_LAYOUT_NAME)
            .resized(SizeConstraint::Full, SizeConstraint::Full),
    );

    update_card_view(siv, card_set);
}

pub fn update_card_view(siv: &mut Cursive, card_set: &CardSet) {
    fn set_optional_row<T>(siv: &mut Cursive, row_name: &str, text: Option<&str>, transform: T)
    where
        T: FnOnce(&str) -> String,
    {
        if let Some(ref mut row) = siv.find_name::<TextView>(row_name) {
            row.set_content(match text {
                Some(txt) => transform(txt),
                _ => String::from(" "),
            });
        }
    }

    if let Some(ref mut main_row) = siv.find_name::<TextView>(MAIN_ROW) {
        main_row.set_content(card_set.get_main_text());
    }
    set_optional_row(siv, ROW_2, card_set.get_pronunciation(), |p| p.to_string());
    set_optional_row(siv, ROW_3, card_set.get_desc(), |description| {
        format!(" Description: {}", description)
    });
    set_optional_row(siv, ROW_4, card_set.get_example(), |example| {
        format!(" Example:     {}", example)
    });
    set_optional_row(siv, SHORTCUTS, card_set.get_shortcuts(), |shortcuts| {
        shortcuts.to_string()
    });
    set_optional_row(siv, TITLE, card_set.get_title(), |name| {
        format!(
            "\n  {}: {}/{}",
            name,
            card_set.current_card + 1,
            card_set.cards_len()
        )
    });
}

fn compose_card_layout() -> LinearLayout {
    LinearLayout::vertical()
        .child(TextView::new("").with_name(TITLE).fixed_height(2))
        .child(
            TextView::new("")
                .center()
                .effect(Effect::Bold)
                .v_align(VAlign::Bottom)
                .with_name(MAIN_ROW)
                .resized(SizeConstraint::Full, SizeConstraint::Full),
        )
        .child(
            LinearLayout::vertical()
                .child(TextView::new(" ").max_height(1))
                .child(TextView::new(" ").center().with_name(ROW_2).fixed_height(2))
                .child(TextView::new(" ").resized(SizeConstraint::Full, SizeConstraint::Full))
                .child(TextView::new(" ").with_name(ROW_3).max_height(4))
                .child(TextView::new(" ").max_height(1))
                .child(TextView::new(" ").with_name(ROW_4).max_height(4))
                .child(TextView::new(" ").max_height(1))
                .child(
                    TextView::new(" ")
                        .h_align(HAlign::Right)
                        .with_name(SHORTCUTS)
                        .fixed_height(2),
                ),
        )
}
