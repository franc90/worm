use cursive::align::HAlign;
use cursive::traits::*;
use cursive::view::SizeConstraint;
use cursive::views::{LinearLayout, NamedView, ResizedView, TextView};

pub const MAIN_LAYOUT_NAME: &str = "MAIN_LAYOUT";

pub fn generate_main_layout() -> ResizedView<NamedView<LinearLayout>>{
    LinearLayout::vertical()
        .child(
            TextView::new("  q: quit | n: next | p: prev | space: turn card  ")
                .h_align(HAlign::Right)
                .fixed_height(2),
        )
        .with_name(MAIN_LAYOUT_NAME)
        .resized(SizeConstraint::Full, SizeConstraint::Full)
}