use std::error::Error;

use mongodb::Collection;

use crate::models::swags_history_model::SwapsHistory;

pub struct SwagsHistoryRepository {
    col: Collection<SwapsHistory>,
}

impl SwagsHistoryRepository {
    pub async fn init(col: Collection<SwapsHistory>) -> Result<Self, Box<dyn Error>> {
        Ok(SwagsHistoryRepository { col })
    }
}
