use std::error::Error;

use mongodb::Collection;

use crate::models::depth_history_model::DepthHistory;

pub struct DepthHistoryRepository {
    col: Collection<DepthHistory>,
}

impl DepthHistoryRepository {
    pub async fn init(col: Collection<DepthHistory>) -> Result<Self, Box<dyn Error>> {
        Ok(DepthHistoryRepository { col })
    }
}
