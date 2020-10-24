use std::cell::RefCell;
use std::rc::Rc;

use serde_json::Result;

use card::card_data;
use card::card_logic;
use test_data::{SET_1, SET_2};

use crate::card::card_data::CardData;
use crate::card::card_ui;

mod card;
mod test_data;

fn main() -> Result<()> {
    let set1: Vec<CardData> = serde_json::from_str(SET_1)?;
    let set2: Vec<CardData> = serde_json::from_str(SET_2)?;

    let card_set1 = card_data::CardSet::new("set1", set1);
    let card_set2 = card_data::CardSet::new("set2", set2);
    let app_data = Rc::new(RefCell::new(card_set2));

    let mut siv = cursive::default();
    siv.set_user_data(app_data);
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(' ', |s| card_logic::reverse_card(s));
    siv.add_global_callback('n', |s| card_logic::next_card(s));
    siv.add_global_callback('p', |s| card_logic::prev_card(s));

    siv.update_theme(|f| f.shadow = false);
    card_ui::setup_deck(&mut siv);

    siv.run();
    Ok(())
}
