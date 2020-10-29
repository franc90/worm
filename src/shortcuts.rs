use cursive::event::Event;
use cursive::views::Dialog;
use cursive::Cursive;

use crate::card::card_logic;
use crate::help;

pub const ALL_SHORTCUTS: &'static [Shortcut] = &[
    Shortcut::Quit,
    Shortcut::ReverseCard,
    Shortcut::PrevCard,
    Shortcut::NextCard,
    Shortcut::TogglePronunciationVisibility,
    Shortcut::ToggleDescriptionVisibility,
    Shortcut::ToggleExampleVisibility,
    Shortcut::ToggleTitleVisibility,
    Shortcut::ToggleShortcutsVisibility,
    Shortcut::ToggleZenMode,
    Shortcut::Help,
];

#[derive(Copy, Clone)]
pub enum Shortcut {
    Quit,
    ReverseCard,
    PrevCard,
    NextCard,
    TogglePronunciationVisibility,
    ToggleDescriptionVisibility,
    ToggleExampleVisibility,
    ToggleTitleVisibility,
    ToggleShortcutsVisibility,
    ToggleZenMode,
    Help,
}
pub trait DisplayEventInHelp {
    fn help_text(&self) -> String;
}

impl DisplayEventInHelp for Event {
    fn help_text(&self) -> String {
        match self {
            Event::Char(' ') => "space".to_string(),
            Event::Char(c) => format!("{}", c),
            Event::AltChar(c) => format!("alt+{}", c),
            Event::CtrlChar(c) => format!("ctrl+{}", c),
            e => panic!("Cannot format {:?} for display in help", e),
        }
    }
}

pub trait ShortcutData {
    fn event(&self) -> Event;
    fn desc(&self) -> String;
    fn call(&self, siv: &mut Cursive);
}

impl ShortcutData for Shortcut {
    fn event(&self) -> Event {
        match self {
            Shortcut::Quit => Event::Char('q'),
            Shortcut::ReverseCard => Event::Char(' '),
            Shortcut::PrevCard => Event::Char('p'),
            Shortcut::NextCard => Event::Char('n'),
            Shortcut::TogglePronunciationVisibility => Event::Char('s'),
            Shortcut::ToggleDescriptionVisibility => Event::Char('d'),
            Shortcut::ToggleExampleVisibility => Event::Char('e'),
            Shortcut::ToggleTitleVisibility => Event::Char('t'),
            Shortcut::ToggleShortcutsVisibility => Event::Char('m'),
            Shortcut::ToggleZenMode => Event::Char('z'),
            Shortcut::Help => Event::Char('?'),
        }
    }

    fn desc(&self) -> String {
        match self {
            Shortcut::Quit => "Quit app or close popup",
            Shortcut::ReverseCard => "Reverse card",
            Shortcut::PrevCard => "Previous card",
            Shortcut::NextCard => "Next card",
            Shortcut::TogglePronunciationVisibility => "Show/hide pronunciation",
            Shortcut::ToggleDescriptionVisibility => "Show/hide description",
            Shortcut::ToggleExampleVisibility => "Show/hide example",
            Shortcut::ToggleTitleVisibility => "Show/hide title",
            Shortcut::ToggleShortcutsVisibility => "Show/hide shortcuts",
            Shortcut::ToggleZenMode => "Toggle zen mode",
            Shortcut::Help => "Show help",
        }
        .to_string()
    }

    fn call(&self, siv: &mut Cursive) {
        match self {
            Shortcut::Quit => back_or_quit(siv),
            Shortcut::ReverseCard => card_logic::reverse_card(siv),
            Shortcut::PrevCard => card_logic::prev_card(siv),
            Shortcut::NextCard => card_logic::next_card(siv),
            Shortcut::TogglePronunciationVisibility => card_logic::toggle_pronunciation(siv),
            Shortcut::ToggleDescriptionVisibility => card_logic::toggle_description(siv),
            Shortcut::ToggleExampleVisibility => card_logic::toggle_example(siv),
            Shortcut::ToggleTitleVisibility => card_logic::toggle_title(siv),
            Shortcut::ToggleShortcutsVisibility => card_logic::toggle_shortcuts(siv),
            Shortcut::ToggleZenMode => card_logic::zen_mode(siv),
            Shortcut::Help => help::show_help(siv),
        }
    }
}

fn back_or_quit(siv: &mut Cursive) {
    match siv.find_name::<Dialog>(help::HELP_DIALOG) {
        None => siv.quit(),
        Some(_) => {
            siv.pop_layer();
        }
    };
}
