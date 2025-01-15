use anyhow::Result;
use clap::Parser;
use config::Config;
use dotenvy::dotenv;
use log::*;
use parser::BankFileParser;
use repo::TransferRepo;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use transfer_service::TransferService;

mod category;
mod config;
mod parser;
mod repo;
mod transfer;
mod transfer_service;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect("Aucun fichier .env");
    info!("processus started");
    let (config, pool) = setup().await?;
    let date = config.get_date()?;

    let transaction_lines = BankFileParser::new(date).parse_file()?;
    let service = TransferService { repo: TransferRepo { pool }};
    let _ = service.import_transactions(&transaction_lines).await;

    service.print_amount_by_category(date).await?;
    println!("===========================================");
    service.print_total_amount(date).await?;
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
