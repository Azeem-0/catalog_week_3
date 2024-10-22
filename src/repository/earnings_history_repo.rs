use std::error::Error;

use mongodb::Collection;

use crate::models::earnings_history_model::EarningsHistory;

pub struct EarningsHistoryRepository {
    col: Collection<EarningsHistory>,
}

impl EarningsHistoryRepository {
    pub async fn init(col: Collection<EarningsHistory>) -> Result<Self, Box<dyn Error>> {
        Ok(EarningsHistoryRepository { col })
    }
}
