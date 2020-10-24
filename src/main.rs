use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::rc::Rc;

use clap::{App, Arg};

use card::card_logic;

use crate::card::card_data::{CardData, CardSet};
use crate::card::card_ui;

mod card;

fn main() {
    let matches = App::new("worm")
        .arg(
            Arg::with_name("INPUT")
                .help("path to json file with words")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input_file_path = matches.value_of("INPUT").unwrap();
    let cards = read_cards_from_file(input_file_path).unwrap();
    let card_set = CardSet::new(input_file_path, cards);
    let card_set = Rc::new(RefCell::new(card_set));

    let mut siv = cursive::default();
    siv.set_user_data(card_set);
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(' ', |s| card_logic::reverse_card(s));
    siv.add_global_callback('n', |s| card_logic::next_card(s));
    siv.add_global_callback('p', |s| card_logic::prev_card(s));

    siv.update_theme(|f| f.shadow = false);
    card_ui::setup_deck(&mut siv);

    siv.run();
}

fn read_cards_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<CardData>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let data = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(data)
}
