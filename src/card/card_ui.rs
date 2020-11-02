use cursive::align::HAlign;
use cursive::align::VAlign;
use cursive::Cursive;
use cursive::theme::{ColorStyle, ColorType, Effect, PaletteColor};
use cursive::traits::*;
use cursive::view::SizeConstraint;
use cursive::views::{Layer, LinearLayout, NamedView, ResizedView, TextView, ViewRef};

use crate::card::card_data::CardSet;

const HINTS_TEXT: &'static str = "| q:quit | ?:help |";
const DESCRIPTION_PREFIX: &'static str = "Description: ";
const EXAMPLE_PREFIX: &'static str = "Example: ";

const TITLE_BAR: &'static str = "title_bar";
const TOP_SPACER: &'static str = "top_spacer";
const TERM: &'static str = "term";
const PRONUNCIATION: &'static str = "pronunciation";
const BOTTOM_SPACER: &'static str = "bottom_spacer";
const DESCRIPTION: &'static str = "description";
const EXAMPLE: &'static str = "example";
const HINTS_BAR: &'static str = "hints_bar";

pub fn setup_layout(siv: &mut Cursive, card_set: &CardSet) {
    siv.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(reverse_color_row(TextView::new(""), TITLE_BAR))
            .child(TextView::new(" ").fixed_height(0).with_name(TOP_SPACER))
            .child(
                TextView::new("")
                    .center()
                    .effect(Effect::Bold)
                    .v_align(VAlign::Bottom)
                    .resized(SizeConstraint::Full, SizeConstraint::Full)
                    .with_name(TERM),
            )
            .child(
                TextView::new("")
                    .center()
                    .v_align(VAlign::Bottom)
                    .fixed_height(2)
                    .with_name(PRONUNCIATION),
            )
            .child(TextView::new(" ").resized(SizeConstraint::Full, SizeConstraint::Full))
            .child(TextView::new(" ").with_name(BOTTOM_SPACER))
            .child(TextView::new(" ").max_height(2).with_name(DESCRIPTION))
            .child(TextView::new(" ").max_height(2).with_name(EXAMPLE))
            .child(reverse_color_row(
                TextView::new(HINTS_TEXT).h_align(HAlign::Right),
                HINTS_BAR,
            ))
            .resized(SizeConstraint::Full, SizeConstraint::Full),
    );

    display(siv, card_set);
}

fn reverse_color_row(view: TextView, view_name: &str) -> Layer<NamedView<ResizedView<TextView>>> {
    Layer::with_color(
        view.effect(Effect::Reverse)
            .fixed_height(1)
            .with_name(view_name),
        ColorStyle::new(
            ColorType::Palette(PaletteColor::Background),
            ColorType::Palette(PaletteColor::Primary),
        ),
    )
}

pub fn display(siv: &mut Cursive, card_set: &CardSet) {
    update_optional_view(siv, TITLE_BAR, &card_set.get_title(), |view, text| {
        let title = format!(
            "{}: {}/{}",
            text,
            card_set.current_card + 1,
            card_set.cards_len()
        );
        let title = if card_set.repeat_current_card() {
            format!("{} TO BE REPEATED", title)
        } else {
            title
        };
        view.get_inner_mut().set_content(title);
        view.set_height(SizeConstraint::Fixed(1));
    });

    let weight = card_set.count_view_weight(
        siv.screen_size().x,
        DESCRIPTION_PREFIX.len(),
        EXAMPLE_PREFIX.len(),
    );

    update_weight(siv, weight, TOP_SPACER);

    if let Some(mut view) = siv.find_name::<ResizedView<TextView>>(TERM) {
        view.get_inner_mut().set_content(card_set.get_main_text())
    }

    update_optional_view(
        siv,
        PRONUNCIATION,
        &card_set.get_pronunciation(),
        |view, text| {
            view.get_inner_mut().set_content(text);
            view.set_height(SizeConstraint::Fixed(2));
        },
    );

    update_weight(siv, -weight, BOTTOM_SPACER);

    update_optional_view(siv, DESCRIPTION, &card_set.get_desc(), |view, text| {
        let description = format!("{}{}", DESCRIPTION_PREFIX, text);
        view.get_inner_mut().set_content(description);
        view.set_height(SizeConstraint::AtMost(2));
    });

    update_optional_view(siv, EXAMPLE, &card_set.get_example(), |view, text| {
        let example = format!("{}{}", EXAMPLE_PREFIX, text);
        view.get_inner_mut().set_content(example);
        view.set_height(SizeConstraint::AtMost(2));
    });

    if let Some(mut view) = siv.find_name::<ResizedView<TextView>>(HINTS_BAR) {
        if card_set.show_hints() {
            view.set_height(SizeConstraint::Fixed(1));
        } else {
            view.set_height(SizeConstraint::Fixed(0));
        }
    }
}

fn update_weight(siv: &mut Cursive, val: i32, name: &str) {
    if let Some(mut view) = siv.find_name::<ResizedView<TextView>>(name) {
        if val > 0 {
            view.set_height(SizeConstraint::Fixed(val as usize));
        } else {
            view.set_height(SizeConstraint::Fixed(0));
        }
    }
}

fn update_optional_view<F>(siv: &mut Cursive, view_name: &str, val: &Option<&str>, cb: F)
where
    F: FnOnce(&mut ViewRef<ResizedView<TextView>>, &str),
{
    if let Some(mut view) = siv.find_name::<ResizedView<TextView>>(view_name) {
        match val {
            None => {
                view.set_height(SizeConstraint::Fixed(0));
            }
            Some(text) => {
                cb(&mut view, *text);
            }
        }
    }
}
