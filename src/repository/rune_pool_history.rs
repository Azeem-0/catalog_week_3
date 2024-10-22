use mongodb::Collection;

use crate::models::rune_pool_history::RunePoolHistory;

pub struct RunePoolHistoryRepository {
    col: Collection<RunePoolHistory>,
}

impl RunePoolHistoryRepository {
    pub async fn init() -> () {
        println!("Intialize depth history repo");
        ()
    }
}
