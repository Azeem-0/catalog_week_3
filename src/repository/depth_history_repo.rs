use futures::{stream::TryStreamExt, StreamExt};
use mongodb::{
    bson::{doc, Document},
    results::InsertOneResult,
    Collection,
};
use std::error::Error;

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

    pub async fn fetch_depth_history_data(
        &self,
        from: f64,
        to: f64,
        count: f64,
    ) -> Result<Vec<DepthHistory>, mongodb::error::Error> {
        let filter = doc! {
            "startTime": { "$gte": from },
            "endTime":{"$lte":to},
        };

        let pipeline = vec![
            doc! { "$match": filter }, // Use the filter here
            doc! { "$group": {
                "_id": {
                    "$toDate": {
                        "$subtract": ["$startTime", { "$mod": ["$startTime", 86400] }]
                    }
                },
                "assetDepth": { "$last": "$assetDepth" },
                "runeDepth": { "$last": "$runeDepth" },
                "assetPrice": { "$last": "$assetPrice" },
                "assetPriceUSD": { "$last": "$assetPriceUSD" },
                "liquidityUnits": { "$last": "$liquidityUnits" },
                "membersCount": { "$last": "$membersCount" },
                "synthUnits": { "$last": "$synthUnits" },
                "synthSupply": { "$last": "$synthSupply" },
                "units": { "$last": "$units" },
                "luvi": { "$last": "$luvi" },
                "startTime": { "$first": "$startTime" },
                "endTime": { "$last": "$endTime" }
            }},
            doc! { "$project": {
                "_id": 0,
                "startTime": 1,
                "endTime": 1,
                "assetDepth": 1,
                "runeDepth": 1,
                "assetPrice": 1,
                "assetPriceUSD": 1,
                "liquidityUnits": 1,
                "membersCount": 1,
                "synthUnits": 1,
                "synthSupply": 1,
                "units": 1,
                "luvi": 1
            }},
            doc! { "$limit": count as i64 },
        ];

        let cursor = self.col.aggregate(pipeline, None).await?;

        let results: Vec<DepthHistory> = cursor
            .try_collect::<Vec<Document>>()
            .await?
            .into_iter()
            .map(|doc| {
                // Convert each Document to DepthHistory
                mongodb::bson::from_document(doc).map_err(|e| {
                    // Convert bson::de::Error to mongodb::error::Error
                    mongodb::error::Error::from(e)
                })
            })
            .collect::<Result<Vec<DepthHistory>, _>>()?;

        Ok(results)

        // let mut cursor = self.col.find(filter, None).await?;

        // let mut results = Vec::new();

        // while let Some(result) = cursor.next().await {
        //     match result {
        //         Ok(document) => results.push(document),
        //         Err(e) => eprintln!("Error fetching document: {:?}", e),
        //     }

        //     if results.len() as f64 >= count {
        //         break;
        //     }
        // }

        // Ok(results)
    }
}
