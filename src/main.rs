use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::rc::Rc;

use clap::{App, Arg, ArgMatches};
use log::info;
use simplelog::{Config, LevelFilter, WriteLogger};

use card::card_logic;

use crate::card::card_data::{CardData, CardSet};
use crate::card::card_ui;
use cursive::event::Event;

mod card;
mod layout;

fn main() {
    set_up_logger();

    let matches = parse_comman_line_args();
    let input_file_path = matches.value_of("INPUT").unwrap();
    let cards = read_cards_from_file(input_file_path).unwrap();
    let card_set = CardSet::new(input_file_path, cards);
    let card_set = Rc::new(RefCell::new(card_set));
    info!("Read card set: {:?}", card_set);

    info!("Setting up cursive");
    let mut siv = cursive::default();
    siv.set_user_data(card_set.clone());
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(' ', |s| card_logic::reverse_card(s));
    siv.add_global_callback('n', |s| card_logic::next_card(s));
    siv.add_global_callback('p', |s| card_logic::prev_card(s));
    siv.add_global_callback('s', |s| card_logic::toggle_show_pronunciation(s));
    siv.add_global_callback('d', |s| card_logic::toggle_show_description(s));
    siv.add_global_callback('e', |s| card_logic::toggle_show_example(s));
    siv.add_global_callback('t', |s| card_logic::toggle_show_title(s));
    siv.add_global_callback('z', |s| card_logic::show_essential(s));

    siv.set_on_pre_event(Event::WindowResize, |s| {
        info!("set_on_pre_event: refresh");
        card_logic::refresh(s)
    });

    siv.update_theme(|f| f.shadow = false);

    siv.add_fullscreen_layer(layout::generate_main_layout());
    card_ui::generate_card_view(&mut siv, &card_set.borrow());

    info!("Started");
    siv.run();
}

fn set_up_logger() {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("worm.log").unwrap(),
    )
    .unwrap();
}

fn parse_comman_line_args<'a>() -> ArgMatches<'a> {
    let matches = App::new("worm")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("INPUT")
                .help("path to json file with words")
                .required(true)
                .index(1),
        )
        .get_matches();
    matches
}

fn read_cards_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<CardData>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}
