use std::cell::RefCell;
use std::rc::Rc;

use serde_json::Result;

use card::card_data;
use card::card_logic;
use select::select_logic;
use select::select_ui;
use test_data::{SET_1, SET_2};

use crate::card::card_data::CardData;
use crate::select::select_data::SelectData;

mod card;
mod select;
mod test_data;

fn main() -> Result<()> {
    let set1: Vec<CardData> = serde_json::from_str(SET_1)?;
    let set2: Vec<CardData> = serde_json::from_str(SET_2)?;

    let card_set1 = card_data::CardSet::new("set1", set1);
    let card_set2 = card_data::CardSet::new("set2", set2);
    let card_sets = vec![card_set1, card_set2];
    let app_data = Rc::new(RefCell::new(SelectData::new(card_sets)));

    let mut siv = cursive::default();
    siv.set_user_data(app_data);
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('b', |s| {
        s.pop_layer();
    });
    siv.add_global_callback(' ', |s| card_logic::reverse_card(s));
    siv.add_global_callback('n', |s| card_logic::next_card(s));
    siv.add_global_callback('p', |s| card_logic::prev_card(s));
    siv.add_global_callback('h', |s| {
        s.pop_layer();
    });
    siv.add_global_callback('j', |s| select_logic::go_down(s));
    siv.add_global_callback('k', |s| select_logic::go_up(s));
    siv.add_global_callback('l', |s| select_logic::show_card_set(s));

    siv.update_theme(|f| f.shadow = false);
    select_ui::setup_view(&mut siv);

    siv.run();
    Ok(())
}
