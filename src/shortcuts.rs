use cursive::Cursive;
use cursive::event::Event;

use crate::card::card_logic;

pub const ALL_CALLBACKS: &'static [Shortcut] = &[
    Shortcut::Quit,
    Shortcut::ReverseCard,
    Shortcut::PrevCard,
    Shortcut::NextCard,
    Shortcut::ToggleShowPronunciation,
    Shortcut::ToggleShowDescription,
    Shortcut::ToggleShowExample,
    Shortcut::ToggleShowTitle,
    Shortcut::ToggleZenMode,
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
        }
    }

    fn desc(&self) -> String {
        match self {
            Shortcut::Quit => "Quit",
            Shortcut::ReverseCard => "Reverse card",
            Shortcut::PrevCard => "Previous card",
            Shortcut::NextCard => "Next card",
            Shortcut::ToggleShowPronunciation => "Toggle show pronunciation",
            Shortcut::ToggleShowDescription => "Toggle show description",
            Shortcut::ToggleShowExample => "Toggle show example",
            Shortcut::ToggleShowTitle => "Toggle show title",
            Shortcut::ToggleZenMode => "Toggle zen mode",
        }
        .to_string()
    }

    fn call(&self, siv: &mut Cursive) {
        match self {
            Shortcut::Quit => siv.quit(),
            Shortcut::ReverseCard => card_logic::reverse_card(siv),
            Shortcut::PrevCard => card_logic::prev_card(siv),
            Shortcut::NextCard => card_logic::next_card(siv),
            Shortcut::ToggleShowPronunciation => card_logic::toggle_show_pronunciation(siv),
            Shortcut::ToggleShowDescription => card_logic::toggle_show_description(siv),
            Shortcut::ToggleShowExample => card_logic::toggle_show_example(siv),
            Shortcut::ToggleShowTitle => card_logic::toggle_show_title(siv),
            Shortcut::ToggleZenMode => card_logic::show_essential(siv),
        }
    }
}
