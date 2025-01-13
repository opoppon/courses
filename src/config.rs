use chrono::{Local, NaiveDate};
use clap::Parser;

#[derive(Clone, Debug, Parser)]

pub struct Config {
    #[arg(long)]
    pub date: Option<NaiveDate>,
}

impl Config {
    pub fn get_date(&self) -> NaiveDate {
        self.date.unwrap_or(Local::now().date_naive())
    }
}
