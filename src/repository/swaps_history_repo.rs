use std::error::Error;

use mongodb::{results::InsertOneResult, Collection};

use crate::models::swaps_history_model::SwapsHistory;

pub struct SwapsHistoryRepository {
    col: Collection<SwapsHistory>,
}

impl SwapsHistoryRepository {
    pub async fn init(col: Collection<SwapsHistory>) -> Result<Self, Box<dyn Error>> {
        Ok(SwapsHistoryRepository { col })
    }

    pub async fn insert_swaps_history(
        &self,
        swaps_history: &SwapsHistory,
    ) -> Result<InsertOneResult, Box<dyn Error>> {
        let insert_details: InsertOneResult = self
            .col
            .insert_one(swaps_history, None)
            .await
            .expect("Failed to insert swaps history.");

        Ok(insert_details)
    }
}
