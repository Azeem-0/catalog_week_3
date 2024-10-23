use std::error::Error;

use mongodb::{results::InsertOneResult, Collection};

use crate::models::earnings_history_model::EarningsHistory;

pub struct EarningsHistoryRepository {
    col: Collection<EarningsHistory>,
}

impl EarningsHistoryRepository {
    pub async fn init(col: Collection<EarningsHistory>) -> Result<Self, Box<dyn Error>> {
        Ok(EarningsHistoryRepository { col })
    }

    pub async fn insert_earnings_history(
        &self,
        earnings_history: &EarningsHistory,
    ) -> Result<InsertOneResult, Box<dyn Error>> {
        let insert_details = self
            .col
            .insert_one(earnings_history, None)
            .await
            .expect("Failed to insert earnings data into database");
        Ok(insert_details)
    }
}
