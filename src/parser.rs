use std::{
    fs::File,
    io::Read,
};

use anyhow::Result;
use chrono::NaiveDate;
use encoding::{all::ISO_8859_1, DecoderTrap, Encoding};

use crate::transfer::Transfer;

pub struct BankFileParser {
    pub date: NaiveDate,
}

impl BankFileParser {
    pub fn new(date: NaiveDate) -> Self {
        Self { date }
    }

    pub fn parse_file(&self) -> Result<Vec<Transfer>> {
        let filename = self.get_filename();
        let mut file = File::open(filename)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let text = ISO_8859_1.decode(&buffer, DecoderTrap::Strict).unwrap();
        let lines: Vec<String> = text.lines().skip(7).map(|line| line.to_string()).collect();

        let mut transfers = vec![];

        for line in lines {
            if let Ok(t) = Transfer::from_str(&line, self.date) {
                transfers.push(t);
            }
        }

        Ok(transfers)
    }

    fn get_filename(&self) -> String {
        let date_str = self.date.format("%Y-%m").to_string();

        format!("{date_str}.tsv")
    }
}
