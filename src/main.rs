use std::cell::RefCell;
use std::rc::Rc;

use cursive::Cursive;
use cursive::traits::*;
use cursive::view::SizeConstraint;
use cursive::views::TextView;
use serde::{Deserialize, Serialize};
use serde_json::Result;

const TERMS: &str = r#"[
  {
    "word": "furnace",
    "translated": "piec (hutniczy) ",
    "explanation": "a container that is heated to a very high temperature and used to heat buildings, melt metal, or burn things",
    "pronunciation": "/ˈfɝː.nɪs/",
    "sentence": "People who work with furnaces in a steel factory need to wear protective clothing.",
    "media_fname": "furnace.mp3",
    "type": "noun",
    "sentence_gap": "People who work with _______s in a steel factory need to wear protective clothing.",
    "gap_term": "furnace"
  },
  {
    "word": "muse",
    "translated": "dumać",
    "explanation": "",
    "pronunciation": "/mjuːz/",
    "sentence": "I was just musing about relationships.",
    "media_fname": "muse.mp3",
    "type": "verb",
    "sentence_gap": "I was just ______ about relationships.",
    "gap_term": "musing"
  }
]"#;

#[derive(Serialize, Deserialize, Debug)]
struct Term {
    word: String,
    translated: String,
    explanation: String,
    pronunciation: String,
    sentence: String,
    media_fname: String,
    #[serde(rename = "type")]
    word_type: String,
    sentence_gap: String,
    gap_term: String,
}

struct Terms {
    terms: Vec<Term>,
    idx: usize,
    reversed: bool,
}

impl Terms {
    fn new(terms: Vec<Term>) -> Terms {
        Terms {
            terms,
            idx: 0,
            reversed: false,
        }
    }

    fn get(&self) -> Option<String> {
        self.terms.get(self.idx).map(|t| {
            if self.reversed {
                t.translated.clone()
            } else {
                t.word.clone()
            }
        })
    }

    fn reverse(&mut self) {
        self.reversed = !self.reversed
    }

    fn next(&mut self) {
        self.idx = if self.idx + 1 < self.terms.len() {
            self.idx + 1
        } else {
            self.idx
        };
    }

    fn prev(&mut self) {
        self.idx = if self.idx > 0 { self.idx - 1 } else { self.idx };
    }
}

fn main() -> Result<()> {
    let terms = Rc::new(RefCell::new(Terms::new(serde_json::from_str(TERMS)?)));
    let mut siv = cursive::default();

    let init_text = terms.borrow().get().unwrap().clone();

    siv.add_global_callback('q', |s| s.quit());
    let reverse_terms = terms.clone();
    let next_term_terms = terms.clone();
    let prev_term_terms = terms.clone();

    siv.add_global_callback(' ', move |s| reverse(s, reverse_terms.clone()));
    siv.add_global_callback('n', move |s| next_term(s, next_term_terms.clone()));
    siv.add_global_callback('p', move |s| prev_term(s, prev_term_terms.clone()));

    siv.update_theme(|f| f.shadow = false);
    siv.add_fullscreen_layer(
        TextView::new(init_text)
            .center()
            .with_name("card")
            .resized(SizeConstraint::Full, SizeConstraint::Full),
    );
    siv.run();
    Ok(())
}

fn reverse(s: &mut Cursive, terms: Rc<RefCell<Terms>>) {
    terms.borrow_mut().reverse();
    update_display(s, terms.clone())
}

fn next_term(s: &mut Cursive, terms: Rc<RefCell<Terms>>) {
    terms.borrow_mut().next();
    update_display(s, terms.clone())
}

fn prev_term(s: &mut Cursive, terms: Rc<RefCell<Terms>>) {
    terms.borrow_mut().prev();
    update_display(s, terms.clone())
}

fn update_display(s: &mut Cursive, terms: Rc<RefCell<Terms>>) {
    let mut view = s.find_name::<TextView>("card").unwrap();
    match terms.borrow().get() {
        Some(val) => view.set_content(val),
        None => (),
    }
}
