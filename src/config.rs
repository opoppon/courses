use std::str::FromStr;

use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct Config {
    ///ex: 2024-12
    #[arg(long)]
    pub date: String,
}

impl Config {
    pub fn get_date(&self) -> Result<NaiveDate> {
        self.parse_date(&self.date)
    }

    fn parse_date(&self, value: &str) -> Result<NaiveDate> {
        let error = Err(anyhow!("Date format is not yyyy-mm-dd".to_string()));

        match value.split("-").collect::<Vec<_>>().as_slice() {
            [_, _, _] => Ok(NaiveDate::from_str(value)?),
            [_, _] => Ok(NaiveDate::from_str(&format!("{value}-01"))?),
            _ => error,
        }
    }
}
