use std::error::Error;

use mongodb::Collection;

use crate::models::rune_pool_history_model::RunePoolHistory;

pub struct RunePoolHistoryRepository {
    col: Collection<RunePoolHistory>,
}

impl RunePoolHistoryRepository {
    pub async fn init(col: Collection<RunePoolHistory>) -> Result<Self, Box<dyn Error>> {
        Ok(RunePoolHistoryRepository { col })
    }
}
