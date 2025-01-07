use anyhow::Result;
use category::SALARY_CAT;
use chrono::NaiveDate;
use clap::Parser;
use config::Config;
use dotenvy::dotenv;
use log::*;
use parser::BankFileParser;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

mod category;
mod config;
mod parser;
mod transfer;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect("Aucun fichier .env");
    info!("processus started");
    let (config, pool) = setup().await?;
    let date = config.get_date();
    let file_parser = BankFileParser::new(date);

    let transfers = file_parser.parse_file()?;
    for transfer in transfers {
        let _ = transfer::create(&transfer, &pool).await;
    }

    let (_, salary) = transfer::get_salary_for_date(date, &pool).await?;

    print_amount_by_category(date, salary, &pool).await?;
    println!("===========================================");
    print_total_amount(date, salary, &pool).await?;
    Ok(())
}

async fn print_amount_by_category(date: NaiveDate, salary: f32, pool: &SqlitePool) -> Result<()> {
    let mut transfers_cat = transfer::get_amount_by_category(date, &pool).await?;
    transfers_cat.push((SALARY_CAT.to_string(), salary));

    for (cat, amount) in transfers_cat {
        println!("{cat} = {amount} euros");
    }

    Ok(())
}

async fn print_total_amount(date: NaiveDate, salary: f32, pool: &SqlitePool) -> Result<()> {
    let (cat, amount) = transfer::get_total_amount(date, &pool).await?;
    let final_amount = salary + amount;

    println!("{cat} = {:.2} euros", final_amount);

    Ok(())
}

async fn setup() -> Result<(Config, SqlitePool)> {
    let config = config::Config::parse();

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let pool = SqlitePoolOptions::new()
        .max_connections(3)
        .connect("sqlite:courses.db")
        .await?;

    Ok((config, pool))
}