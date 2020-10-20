use std::cell::RefCell;
use std::rc::Rc;

use serde_json::Result;

mod card;
mod data;
mod test_data;

use card::card_logic;
use card::card_data;
use card::card_ui;
use test_data::{SET_1, SET_2};

fn main() -> Result<()> {
    let set1 = serde_json::from_str(SET_1)?;
    let set2 = serde_json::from_str(SET_2)?;
    let cardDeck = Rc::new(RefCell::new(card_data::CardSet::new(set1)));

    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    siv.update_theme(|f| f.shadow = false);
    card_ui::setup_deck(&mut siv, cardDeck);

    siv.run();
    Ok(())
}

