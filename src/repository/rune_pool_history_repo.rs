use std::error::Error;

use mongodb::{results::InsertOneResult, Collection};

use crate::models::rune_pool_history_model::RunePoolHistory;

pub struct RunePoolHistoryRepository {
    col: Collection<RunePoolHistory>,
}

impl RunePoolHistoryRepository {
    pub async fn init(col: Collection<RunePoolHistory>) -> Result<Self, Box<dyn Error>> {
        Ok(RunePoolHistoryRepository { col })
    }
    pub async fn insert_rune_pool_history(
        &self,
        rune_pool_history: &RunePoolHistory,
    ) -> Result<InsertOneResult, Box<dyn Error>> {
        let insert_details = self
            .col
            .insert_one(rune_pool_history, None)
            .await
            .expect("Failed to insert Rune Pool history.");

        Ok(insert_details)
    }
}
