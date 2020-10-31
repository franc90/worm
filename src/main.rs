use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
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

fn main() {
    let matches = parse_comman_line_args();
    let input_file_path = matches.value_of("INPUT").unwrap();
    let shuffle = matches.is_present("shuffle");

    if matches.is_present("verbose") {
        set_up_logger();
    }

    let cards = read_cards_from_file(input_file_path, shuffle).unwrap();
    let card_set = CardSet::new(input_file_path, cards);
    let card_set = Rc::new(RefCell::new(card_set));
    info!("Read card set: {}", input_file_path);

    info!("Setting up cursive");
    let mut siv = cursive::default();
    siv.set_user_data(card_set.clone());
    shortcuts::ALL_SHORTCUTS.iter().for_each(|shortcut| {
        let shortcut = shortcut.clone();
        shortcut.event().iter().for_each(|event| {
            siv.add_global_callback(event.clone(), move |siv| shortcut.call(siv));
        })
    });

    siv.set_on_pre_event(Event::WindowResize, |s| {
        info!("set_on_pre_event: refresh");
        card_logic::refresh(s)
    });

    siv.update_theme(|f| f.shadow = false);

    card_ui::setup_layout(&mut siv, &card_set.borrow());

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
        .arg(
            Arg::with_name("shuffle")
                .short("s")
                .long("shuffle")
                .help("shuffle input to create unique experience"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("turns on logging"),
        )
        .get_matches();
    matches
}

fn read_cards_from_file<P: AsRef<Path>>(
    path: P,
    shuffle: bool,
) -> Result<Vec<CardData>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut data: Vec<CardData> = serde_json::from_reader(reader)?;
    if shuffle {
        info!("shuffling card set");
        data.shuffle(&mut thread_rng());
    }
    Ok(data)
}
