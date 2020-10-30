use cursive::theme::Effect;
use cursive::traits::*;
use cursive::views::{Dialog, LinearLayout, TextView};
use cursive::Cursive;

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
        let texts: Vec<String> = shortcut.event().iter().map(|e| e.help_text()).collect();

        layout.add_child(TextView::new(format!(
            "{:7}   {}",
            texts.join("/"),
            shortcut.desc()
        )))
    });
    siv.add_layer(Dialog::around(layout.scrollable()).with_name(HELP_DIALOG));
}
