use anyhow::Result;
use chrono::NaiveDate;
use sqlx::prelude::FromRow;

#[derive(Clone, Debug, Default, FromRow)]
pub struct Transfer {
    pub date: NaiveDate,
    pub label: String,
    pub _category: Option<String>,
    pub amount: f32,
}

impl Transfer {
    pub fn from_str(s: &str, default_date: NaiveDate) -> Result<Self> {
        let parts: Vec<&str> = s.split('\t').collect();
        let label = parts[1].replace("\"", "").to_string();
        let date = parse_date(parts[0], default_date);
        let value: f32 = parts[2].replace(",", ".").parse()?;

        let result = Transfer {
            date,
            label,
            amount: value,
            ..Default::default()
        };
        Ok(result)
    }
}

fn parse_date(value: &str, default_date: NaiveDate) -> NaiveDate {
    NaiveDate::parse_from_str(value, "%d/%m/%Y").unwrap_or(default_date)
}
