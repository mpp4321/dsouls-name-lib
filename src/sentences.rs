use crate::util::{words::*, pick_from};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Debug)]
pub struct Sentence {
    inner: String,
}

impl Sentence {
    /// Create new sentence
    pub fn new(inner: &str) -> Self {
        Sentence { inner: inner.to_string() }
    }

    /// Put's a string into a slot in the Sentence
    ///
    /// Example sentence: {adjective} {noun}
    /// replace_slot("adjective", "nice") will put "nice" into the first adjective slot
    pub fn replace_slot(&mut self, slot: &str, inner: String) {
        let slot_formatted = format!("{{{}}}", slot);
        self.inner = self.inner.replacen(&slot_formatted, &inner, 1);
    }

    pub fn get_modified(&self) -> String {
        self.inner.clone()
    }

    /// Returns the next slot in the modified sentence
    pub fn get_next_slot(&self) -> Option<String> {
        let open_indx = self.inner.find('{');
        let close_indx = self.inner.find('}');

        if open_indx.is_none() || close_indx.is_none() {
            return None;
        }

        Some(self.inner[open_indx.unwrap() + 1..close_indx.unwrap()].to_string())
    }
}

pub fn load_sentences_from_file(file_buf: &mut BufReader<File>) -> Vec<Sentence> {
    let mut sentences = Vec::new();
    for line in file_buf.lines() {
        let line = line.unwrap();
        let sentence = Sentence::new(&line);
        sentences.push(sentence);
    }
    sentences
}

pub fn generate_dynamic_sentence(mut sentence: Sentence) -> String {
    let nouns = NOUNS.clone();
    let adjectives = ADJECTIVES.clone();
    let titles = TITLES.clone();
    let places = PLACES.clone();

    let mut next_slot = sentence.get_next_slot();
    loop {
        match next_slot {
            Some(val) => match val.as_str() {
                "noun" => {
                    let noun = pick_from(&nouns);
                    sentence.replace_slot("noun", noun.clone());
                }
                "adjective" => {
                    let adjective = pick_from(&adjectives);
                    sentence.replace_slot("adjective", adjective.clone());
                }
                "title" => {
                    let title = pick_from(&titles);
                    sentence.replace_slot("title", title.clone());
                }
                "place" => {
                    let place = pick_from(&places);
                    sentence.replace_slot("place", place.clone());
                }
                _ => break,
            },
            None => break,
        }
        next_slot = sentence.get_next_slot();
    }
    return sentence.get_modified();
}
