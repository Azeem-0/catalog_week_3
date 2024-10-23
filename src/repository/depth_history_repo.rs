use std::error::Error;

use mongodb::{bson::doc, options::DeleteOptions, results::InsertOneResult, Collection};

use crate::models::depth_history_model::DepthHistory;

pub struct DepthHistoryRepository {
    col: Collection<DepthHistory>,
}

impl DepthHistoryRepository {
    pub async fn init(col: Collection<DepthHistory>) -> Result<Self, Box<dyn Error>> {
        Ok(DepthHistoryRepository { col })
    }

    pub async fn insert_depth_history(
        &self,
        depth_history: &DepthHistory,
    ) -> Result<InsertOneResult, Box<dyn Error>> {
        let insert_details = self
            .col
            .insert_one(depth_history, None)
            .await
            .expect("Failed to insert depth history.");

        Ok(insert_details)
    }
}
