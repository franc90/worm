use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::rc::Rc;

use clap::{App, Arg, ArgMatches};
use cursive::event::Event;
use log::info;
use rand::seq::SliceRandom;
use rand::thread_rng;
use simplelog::{Config, LevelFilter, WriteLogger};

use card::card_logic;

use crate::card::card_data::{CardData, CardSet};
use crate::card::card_ui;
use crate::shortcuts::ShortcutData;

mod card;
mod help;
mod shortcuts;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = parse_comman_line_args();
    if let Some(file) = matches.value_of("debug") {
        set_up_logger(file);
    }

    let input_file = matches.value_of("input");
    let shuffle_cards = matches.is_present("shuffle");
    let card_set = read_card_set(input_file, shuffle_cards)?;

    info!("Setting up cursive");
    let mut siv = cursive::default();
    siv.set_user_data(card_set.clone());
    shortcuts::ALL_SHORTCUTS.iter().for_each(|shortcut| {
        let shortcut = shortcut.clone();
        shortcut.event().iter().for_each(|event| {
            info!("..add callback for key {:?}: {}", event, shortcut.desc());
            siv.add_global_callback(event.clone(), move |siv| shortcut.call(siv));
        })
    });

    siv.set_on_pre_event(Event::WindowResize, |s| {
        info!("WindowResize: refreshing");
        card_logic::refresh(s)
    });

    siv.update_theme(|f| f.shadow = false);
    card_ui::setup_layout(&mut siv, &card_set.borrow());
    info!("Cursive set up! Starting");
    siv.run();

    Ok(())
}

fn parse_comman_line_args<'a>() -> ArgMatches<'a> {
    let matches = App::new("worm")
        .version(env!("CARGO_PKG_VERSION"))
        .about("WORd Memorizer")
        .arg(
            Arg::with_name("input")
                .value_name("INPUT")
                .help("JSON file with content; if not provided stdin will be used"),
        )
        .arg(
            Arg::with_name("shuffle")
                .short("s")
                .long("shuffle")
                .help("Shuffle input to create unique experience"),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .value_name("DEBUG_FILE")
                .help("Debug file path; turns on logging"),
        )
        .get_matches();
    matches
}

fn set_up_logger(debug_file: &str) {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create(debug_file).unwrap(),
    )
    .unwrap();
}

fn read_card_set(
    input_file: Option<&str>,
    shuffle_cards: bool,
) -> Result<Rc<RefCell<CardSet>>, Box<dyn Error>> {
    Ok(Rc::new(RefCell::new(match input_file {
        Some(path) => {
            let cards = read_cards_from_file(path, shuffle_cards)?;
            info!("Read set of {} cards from file: {}", cards.len(), path);
            CardSet::new(path, cards)
        }
        None => {
            info!("Reading from stdin");
            let cards = read_cards_from_stdin(shuffle_cards)?;
            info!("Read set of {} cards from stdin", cards.len());
            CardSet::new("stdin", cards)
        }
    })))
}

fn read_cards_from_file<P: AsRef<Path>>(
    path: P,
    shuffle_cards: bool,
) -> Result<Vec<CardData>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader).map(|cards| shuffle(cards, shuffle_cards))?)
}

fn read_cards_from_stdin(shuffle_cards: bool) -> Result<Vec<CardData>, Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(serde_json::from_str(&buffer).map(|cards| shuffle(cards, shuffle_cards))?)
}

fn shuffle(mut data: Vec<CardData>, shuffle_cards: bool) -> Vec<CardData> {
    if shuffle_cards {
        info!("shuffling card set");
        data.shuffle(&mut thread_rng());
    }
    data
}
