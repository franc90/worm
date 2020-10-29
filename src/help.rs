use cursive::Cursive;
use cursive::theme::Effect;
use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, TextView};

use crate::shortcuts;
use crate::shortcuts::{DisplayEventInHelp, ShortcutData};

pub const HELP_DIALOG: &str = "help_dialog";

pub fn show_help(siv: &mut Cursive) {
    if let Some(_) = siv.find_name::<Dialog>(HELP_DIALOG) {
        return;
    }

    let mut layout =
        LinearLayout::vertical().child(TextView::new("Help").effect(Effect::Bold).fixed_height(2));

    shortcuts::ALL_SHORTCUTS.iter().for_each(|shortcut| {
        layout.add_child(TextView::new(format!(
            "{:5}   {}",
            shortcut.event().help_text(),
            shortcut.desc()
        )))
    });
    siv.add_layer(Dialog::around(layout.scrollable()).with_name(HELP_DIALOG));
}
