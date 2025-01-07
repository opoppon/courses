use std::fs;

use anyhow::Result;
use chrono::NaiveDate;

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
        let lines = fs::read_to_string(filename)?;
        let lines = lines.lines().skip(7);

        let mut transfers = vec![];

        for line in lines {
            if let Ok(t) = Transfer::from_str(line, self.date) {
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
