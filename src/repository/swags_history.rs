use mongodb::Collection;

use crate::models::swags_history::SwapsHistory;

pub struct SwagsHistoryRepository {
    col: Collection<SwapsHistory>,
}

impl SwagsHistoryRepository {
    pub async fn init() -> () {
        println!("Intialize depth history repo");
        ()
    }
}
