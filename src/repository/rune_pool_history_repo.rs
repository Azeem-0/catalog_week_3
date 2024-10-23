use std::error::Error;

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    results::InsertOneResult,
    Collection,
};

use crate::{models::rune_pool_history_model::RunePoolHistory, utils::time_interval::TimeInterval};

pub struct RunePoolHistoryRepository {
    col: Collection<RunePoolHistory>,
}

impl RunePoolHistoryRepository {
    pub async fn init(col: Collection<RunePoolHistory>) -> Result<Self, Box<dyn Error>> {
        Ok(RunePoolHistoryRepository { col })
    }
    pub async fn insert_rune_pool_history(
        &self,
        rune_pool_history: &RunePoolHistory,
    ) -> Result<InsertOneResult, Box<dyn Error>> {
        let insert_details = self
            .col
            .insert_one(rune_pool_history, None)
            .await
            .expect("Failed to insert Rune Pool history.");

        Ok(insert_details)
    }

    pub async fn fetch_rune_pool_history_data(
        &self,
        from: f64,
        to: f64,
        count: f64,
        interval: TimeInterval,
        page: i64,
        sort_by: String,
    ) -> Result<Vec<RunePoolHistory>, mongodb::error::Error> {
        let filter = doc! {
            "startTime": { "$gte": from },
            "endTime":{"$lte":to},
        };

        let mut sort_by = sort_by;

        if !RunePoolHistory::has_field(&sort_by) {
            sort_by = String::from("startTime");
        }

        let sort_stage = doc! { &sort_by: -1 };

        let skip = (page - 1).max(0) * (count as i64);

        let interval_seconds = interval.as_seconds();

        let pipeline = vec![
            doc! { "$match": filter },
            doc! { "$sort": { "startTime": 1 } },
            doc! {
                "$group": {
                    "_id": {
                        "$toDate": {
                            "$subtract": [
                                "$startTime",
                                { "$mod": ["$startTime", interval_seconds] }
                            ]
                        }
                    },
                    "count": { "$last": "$count" },
                    "units": { "$last": "$units" },
                    "startTime": { "$first": "$startTime" },
                    "endTime": { "$last": "$endTime" }
                }
            },
            doc! { "$project": {
                "_id": 0,
                "startTime": 1,
                "endTime": 1,
                "count": 1,
                "units": 1
            }},
            doc! { "$sort": { "startTime": 1 } },
            doc! { "$skip": skip },
            doc! { "$limit": count as i64 },
            doc! { "$sort": sort_stage },
        ];

        let cursor = self.col.aggregate(pipeline, None).await?;

        let results: Vec<RunePoolHistory> = cursor
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
            .collect::<Result<Vec<RunePoolHistory>, _>>()?;

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
