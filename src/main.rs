use serde::{Deserialize, Serialize};
use serde_json::Result;

const TERM: &str = r#"{
  "word": "muse",
  "translated": "dumać",
  "explanation": "",
  "pronunciation": "/mjuːz/",
  "sentence": "I was just musing about relationships.",
  "media_fname": "muse.mp3",
  "type": "verb",
  "sentence_gap": "I was just ______ about relationships.",
  "gap_term": "musing"
}"#;
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

fn main() -> Result<()> {
    println!("Hello, world!\n{}", TERMS);

    // let term: Term = serde_json::from_str(TERM)?;
    // println!("Read term: {:?}", term);
    let terms: Vec<Term> = serde_json::from_str(TERMS)?;
    println!("Read term: {:?}", terms);
    Ok(())
}
