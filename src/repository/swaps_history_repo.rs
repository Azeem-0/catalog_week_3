use std::error::Error;

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    results::InsertOneResult,
    Collection,
};

use crate::{models::swaps_history_model::SwapsHistory, utils::time_interval::TimeInterval};

pub struct SwapsHistoryRepository {
    col: Collection<SwapsHistory>,
}

impl SwapsHistoryRepository {
    pub async fn init(col: Collection<SwapsHistory>) -> Result<Self, Box<dyn Error>> {
        Ok(SwapsHistoryRepository { col })
    }

    pub async fn insert_swaps_history(
        &self,
        swaps_history: &SwapsHistory,
    ) -> Result<InsertOneResult, Box<dyn Error>> {
        let insert_details: InsertOneResult = self
            .col
            .insert_one(swaps_history, None)
            .await
            .expect("Failed to insert swaps history.");

        Ok(insert_details)
    }

    pub async fn fetch_swaps_history_data(
        &self,
        from: f64,
        to: f64,
        count: f64,
        interval: TimeInterval,
        page: i64,
        sort_by: String,
    ) -> Result<Vec<SwapsHistory>, mongodb::error::Error> {
        let filter = doc! {
            "startTime": { "$gte": from },
            "endTime":{"$lte":to},
        };

        let mut sort_by = sort_by;

        if !SwapsHistory::has_field(&sort_by) {
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
                    "toAssetCount": { "$last": "$toAssetCount" },
                    "toRuneCount": { "$last": "$toRuneCount" },
                    "toTradeCount": { "$last": "$toTradeCount" },
                    "fromTradeCount": { "$last": "$fromTradeCount" },
                    "synthMintCount": { "$last": "$synthMintCount" },
                    "synthRedeemCount": { "$last": "$synthRedeemCount" },
                    "totalCount": { "$last": "$totalCount" },
                    "toAssetVolume": { "$last": "$toAssetVolume" },
                    "toRuneVolume": { "$last": "$toRuneVolume" },
                    "toTradeVolume": { "$last": "$toTradeVolume" },
                    "fromTradeVolume": { "$last": "$fromTradeVolume" },
                    "synthMintVolume": { "$last": "$synthMintVolume" },
                    "synthRedeemVolume": { "$last": "$synthRedeemVolume" },
                    "totalVolume": { "$last": "$totalVolume" },
                    "toAssetVolumeUSD": { "$last": "$toAssetVolumeUSD" },
                    "toRuneVolumeUSD": { "$last": "$toRuneVolumeUSD" },
                    "toTradeVolumeUSD": { "$last": "$toTradeVolumeUSD" },
                    "fromTradeVolumeUSD": { "$last": "$fromTradeVolumeUSD" },
                    "synthMintVolumeUSD": { "$last": "$synthMintVolumeUSD" },
                    "synthRedeemVolumeUSD": { "$last": "$synthRedeemVolumeUSD" },
                    "totalVolumeUSD": { "$last": "$totalVolumeUSD" },
                    "toAssetFees": { "$last": "$toAssetFees" },
                    "toRuneFees": { "$last": "$toRuneFees" },
                    "toTradeFees": { "$last": "$toTradeFees" },
                    "fromTradeFees": { "$last": "$fromTradeFees" },
                    "synthMintFees": { "$last": "$synthMintFees" },
                    "synthRedeemFees": { "$last": "$synthRedeemFees" },
                    "totalFees": { "$last": "$totalFees" },
                    "toAssetAverageSlip": { "$last": "$toAssetAverageSlip" },
                    "toRuneAverageSlip": { "$last": "$toRuneAverageSlip" },
                    "toTradeAverageSlip": { "$last": "$toTradeAverageSlip" },
                    "fromTradeAverageSlip": { "$last": "$fromTradeAverageSlip" },
                    "synthMintAverageSlip": { "$last": "$synthMintAverageSlip" },
                    "synthRedeemAverageSlip": { "$last": "$synthRedeemAverageSlip" },
                    "averageSlip": { "$last": "$averageSlip" },
                    "runePriceUSD": { "$last": "$runePriceUSD" },
                    "startTime": { "$first": "$startTime" },
                    "endTime": { "$last": "$endTime" }
                }
            },
            doc! { "$project": {
                "_id": 0,
                "startTime": 1,
                "endTime": 1,
                "toAssetCount": 1,
                "toRuneCount": 1,
                "toTradeCount": 1,
                "fromTradeCount": 1,
                "synthMintCount": 1,
                "synthRedeemCount": 1,
                "totalCount": 1,
                "toAssetVolume": 1,
                "toRuneVolume": 1,
                "toTradeVolume": 1,
                "fromTradeVolume": 1,
                "synthMintVolume": 1,
                "synthRedeemVolume": 1,
                "totalVolume": 1,
                "toAssetVolumeUSD": 1,
                "toRuneVolumeUSD": 1,
                "toTradeVolumeUSD": 1,
                "fromTradeVolumeUSD": 1,
                "synthMintVolumeUSD": 1,
                "synthRedeemVolumeUSD": 1,
                "totalVolumeUSD": 1,
                "toAssetFees": 1,
                "toRuneFees": 1,
                "toTradeFees": 1,
                "fromTradeFees": 1,
                "synthMintFees": 1,
                "synthRedeemFees": 1,
                "totalFees": 1,
                "toAssetAverageSlip": 1,
                "toRuneAverageSlip": 1,
                "toTradeAverageSlip": 1,
                "fromTradeAverageSlip": 1,
                "synthMintAverageSlip": 1,
                "synthRedeemAverageSlip": 1,
                "averageSlip": 1,
                "runePriceUSD": 1
            }},
            doc! {"$sort" : {"startTime" : 1}},
            doc! {"$skip": skip},
            doc! { "$limit": count as i64 },
            doc! {"$sort" : sort_stage},
        ];

        let cursor = self.col.aggregate(pipeline, None).await?;

        let results: Vec<SwapsHistory> = cursor
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
            .collect::<Result<Vec<SwapsHistory>, _>>()?;

        Ok(results)
    }
}
