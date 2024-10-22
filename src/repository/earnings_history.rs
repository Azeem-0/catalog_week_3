use mongodb::Collection;

use crate::models::earnings_history::EarningsHistory;

pub struct EarningsHistoryRepository {
    col: Collection<EarningsHistory>,
}

impl EarningsHistoryRepository {
    pub async fn init() -> () {
        println!("Intialize depth history repo");
        ()
    }
}
