use std::{collections::HashMap, error::Error};

use csv::ReaderBuilder;
use rand::prelude::IndexedRandom;

#[derive(Debug, Clone)]
pub struct OpeningBook
{
    book: HashMap<String, Vec<String>>,
}

impl OpeningBook
{
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>>
    {
        let mut rdr = ReaderBuilder::new().delimiter(b',').from_path(path)?;

        let mut book: HashMap<String, Vec<String>> = HashMap::new();

        for result in rdr.records()
        {
            let record = result?;
            let epd = record.get(0).unwrap().trim().to_string();
            let moves_str = record.get(1).unwrap().trim();

            let moves: Vec<String> = moves_str
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            book.entry(epd).or_default().extend(moves);
        }

        Ok(Self { book })
    }

    pub fn get_move(&self, fen: &str) -> Option<String>
    {
        return self.book.get(fen).and_then(|mvs| {
            let mut rng = rand::rng();
            return mvs.choose(&mut rng).cloned();
        });
    }
}
