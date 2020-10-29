use cursive::Cursive;
use cursive::event::Event;
use cursive::views::Dialog;

use crate::card::card_logic;
use crate::help;

pub const ALL_SHORTCUTS: &'static [Shortcut] = &[
    Shortcut::Quit,
    Shortcut::ReverseCard,
    Shortcut::PrevCard,
    Shortcut::NextCard,
    Shortcut::ToggleShowPronunciation,
    Shortcut::ToggleShowDescription,
    Shortcut::ToggleShowExample,
    Shortcut::ToggleShowTitle,
    Shortcut::ToggleZenMode,
    Shortcut::Help,
];

#[derive(Copy, Clone)]
pub enum Shortcut {
    Quit,
    ReverseCard,
    PrevCard,
    NextCard,
    ToggleShowPronunciation,
    ToggleShowDescription,
    ToggleShowExample,
    ToggleShowTitle,
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
            Shortcut::ToggleShowPronunciation => Event::Char('s'),
            Shortcut::ToggleShowDescription => Event::Char('d'),
            Shortcut::ToggleShowExample => Event::Char('e'),
            Shortcut::ToggleShowTitle => Event::Char('t'),
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
            Shortcut::ToggleShowPronunciation => "Toggle show pronunciation",
            Shortcut::ToggleShowDescription => "Toggle show description",
            Shortcut::ToggleShowExample => "Toggle show example",
            Shortcut::ToggleShowTitle => "Toggle show title",
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
            Shortcut::ToggleShowPronunciation => card_logic::toggle_show_pronunciation(siv),
            Shortcut::ToggleShowDescription => card_logic::toggle_show_description(siv),
            Shortcut::ToggleShowExample => card_logic::toggle_show_example(siv),
            Shortcut::ToggleShowTitle => card_logic::toggle_show_title(siv),
            Shortcut::ToggleZenMode => card_logic::show_essential(siv),
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
