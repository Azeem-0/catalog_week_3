use std::error::Error;

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    results::InsertOneResult,
    Collection,
};

use crate::{
    models::earnings_history_model::{EarningsHistory, EarningsHistoryPool},
    utils::time_interval::TimeInterval,
};

pub struct EarningsHistoryRepository {
    col: Collection<EarningsHistory>,
    pools_col: Collection<EarningsHistoryPool>,
}

impl EarningsHistoryRepository {
    pub async fn init(
        col: Collection<EarningsHistory>,
        pools_col: Collection<EarningsHistoryPool>,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(EarningsHistoryRepository { col, pools_col })
    }

    pub async fn insert_earnings_history(
        &self,
        earnings_history: &EarningsHistory,
    ) -> Result<InsertOneResult, Box<dyn Error>> {
        let insert_details = self
            .col
            .insert_one(earnings_history, None)
            .await
            .expect("Failed to insert earnings data into database");
        Ok(insert_details)
    }

    pub async fn insert_earnings_history_pool(
        &self,
        pool: &EarningsHistoryPool,
    ) -> Result<InsertOneResult, Box<dyn Error>> {
        let insert_details = self
            .pools_col // Assuming you have a separate collection for pools
            .insert_one(pool, None)
            .await
            .map_err(|e| e)?;
        Ok(insert_details)
    }

    pub async fn fetch_earnings_history_data(
        &self,
        from: f64,
        to: f64,
        count: f64,
        interval: TimeInterval,
        page: i64,
        sort_by: String,
        _: String,
    ) -> Result<Vec<EarningsHistory>, mongodb::error::Error> {
        let filter = doc! {
            "startTime": { "$gte": from },
            "endTime":{"$lte":to},
        };

        let mut sort_by = sort_by;

        if !EarningsHistory::has_field(&sort_by) {
            sort_by = String::from("startTime");
        }

        let _ = doc! { &sort_by: -1 };

        let skip = (page - 1).max(0) * (count as i64);

        let interval_seconds = interval.as_seconds();

        let pipeline = vec![
            doc! { "$match": filter },
            doc! {"$sort" : {"startTime" : 1}},
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
                    "startTime": { "$first": "$startTime" },
                    "endTime": { "$last": "$endTime" },
                    "liquidityFees": { "$last": "$liquidityFees" },
                    "blockRewards": { "$last": "$blockRewards" },
                    "earnings": { "$last": "$earnings" },
                    "bondingEarnings": { "$last": "$bondingEarnings" },
                    "liquidityEarnings": { "$last": "$liquidityEarnings" },
                    "avgNodeCount": { "$last": "$avgNodeCount" },
                    "runePriceUSD": { "$last": "$runePriceUSD" },
                    "pools": { "$last": "$pools" }
                }
            },
            doc! {"$skip": skip},
            doc! { "$limit": count as i64 },
        ];

        let cursor = self.col.aggregate(pipeline, None).await?;

        let results: Vec<EarningsHistory> = cursor
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
            .collect::<Result<Vec<EarningsHistory>, _>>()?;

        Ok(results)
    }
}
