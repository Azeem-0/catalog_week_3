use mongodb::Collection;

use crate::models::depth_history::DepthHistory;

pub struct DepthHistoryRepository {
    col: Collection<DepthHistory>,
}

impl DepthHistoryRepository {
    pub async fn init() -> () {
        println!("Intialize depth history repo");
        ()
    }
}
