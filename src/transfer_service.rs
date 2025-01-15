use anyhow::Result;
use chrono::NaiveDate;

use crate::{repo::TransferRepo, transfer::Transfer};

pub struct TransferService {
    pub repo: TransferRepo
}

impl TransferService {
    pub async fn import_transactions(&self, transfers: &[Transfer]) -> Result<()> {
        for transfer in transfers {
            let _ = self.repo.create(&transfer).await;
        }
    
        Ok(())
    }
    
    pub async fn print_amount_by_category(&self, date: NaiveDate) -> Result<()> {
        let transfers_cat = self.repo.get_amount_by_category(date).await?;
    
        for (cat, amount) in transfers_cat {
            println!("{cat} = {amount} euros");
        }
    
        Ok(())
    }
    
    pub async fn print_total_amount(&self, date: NaiveDate) -> Result<()> {
        let (cat, amount) = self.repo.get_total_amount(date).await?;
    
        println!("{cat} = {:.2} euros", amount);
    
        Ok(())
    }    
}