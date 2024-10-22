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

    pub async fn temp_delete(&self) -> Result<(), mongodb::error::Error> {
        // Define your query here to match the documents you want to delete
        let query = doc! { "start_time": { "$gt": "1729602378" } }; // Example query

        // Optional delete options
        let delete_options = DeleteOptions::builder().build();

        // Perform the delete operation
        match self.col.delete_many(query, delete_options).await {
            Ok(result) => {
                println!("{} documents deleted", result.deleted_count);
                Ok(())
            }
            Err(e) => {
                eprintln!("Error occurred while deleting documents: {:?}", e);
                Err(e)
            }
        }
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
