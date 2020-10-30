use cursive::align::HAlign;
use cursive::align::VAlign;
use cursive::Cursive;
use cursive::theme::{ColorStyle, ColorType, Effect, PaletteColor};
use cursive::traits::*;
use cursive::view::SizeConstraint;
use cursive::views::{Layer, LinearLayout, ResizedView, TextView};

use crate::card::card_data::CardSet;

const CARD_LAYOUT_NAME: &str = "card_layout";
pub const SHORTCUTS_TEXT: &str = "| q (quit) | ? (help) ";
pub const DESCRIPTION_PREFIX: &str = "Description: ";
pub const EXAMPLE_PREFIX: &str = "Example: ";

pub fn display(siv: &mut Cursive, card_set: &CardSet) {
    if let Some(_) = siv.find_name::<LinearLayout>(CARD_LAYOUT_NAME) {
        siv.pop_layer();
    }
    let mut layout = LinearLayout::vertical();

    card_set.get_title().iter().for_each(|title| {
        layout.add_child(reverse_color_row(TextView::new(format!(
            "{}: {}/{}",
            *title,
            card_set.current_card + 1,
            card_set.cards_len()
        ))));
    });

    let weight = card_set.count_view_weight(
        siv.screen_size().x,
        DESCRIPTION_PREFIX.len(),
        EXAMPLE_PREFIX.len(),
    );

    if weight > 0 {
        layout.add_child(TextView::new(" ").fixed_height(weight as usize));
    }

    layout.add_child(create_term_row(card_set.get_main_text()));
    card_set.get_pronunciation().iter().for_each(|pron| {
        layout.add_child(TextView::new(" ").fixed_height(1));
        layout.add_child(TextView::new(*pron).center().fixed_height(1));
    });

    layout.add_child(TextView::new(" ").resized(SizeConstraint::Full, SizeConstraint::Full));

    card_set.get_desc().iter().for_each(|desc| {
        layout.add_child(TextView::new(format!(" Description: {}", *desc)).max_height(2))
    });

    card_set.get_example().iter().for_each(|example| {
        layout.add_child(TextView::new(format!(" Example: {}", *example)).max_height(2))
    });

    if weight < 0 {
        layout.add_child(TextView::new(" ").fixed_height(-weight as usize));
    }

    card_set.get_shortcuts().iter().for_each(|shortcuts| {
        layout.add_child(reverse_color_row(
            TextView::new(*shortcuts).h_align(HAlign::Right),
        ))
    });

    siv.add_fullscreen_layer(
        layout
            .with_name(CARD_LAYOUT_NAME)
            .resized(SizeConstraint::Full, SizeConstraint::Full),
    );
}

fn reverse_color_row(view: TextView) -> Layer<ResizedView<TextView>> {
    Layer::with_color(
        view.effect(Effect::Reverse).fixed_height(1),
        ColorStyle::new(
            ColorType::Palette(PaletteColor::Background),
            ColorType::Palette(PaletteColor::Primary),
        ),
    )
}

fn create_term_row(value: &str) -> ResizedView<TextView> {
    TextView::new(value)
        .center()
        .effect(Effect::Bold)
        .v_align(VAlign::Bottom)
        .resized(SizeConstraint::Full, SizeConstraint::Full)
}
