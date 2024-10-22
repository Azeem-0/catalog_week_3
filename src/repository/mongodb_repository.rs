use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{Client, Collection};

use crate::models::{
    depth_history_model::DepthHistory, earnings_history_model::EarningsHistory,
    rune_pool_history_model::RunePoolHistory, swaps_history_model::SwapsHistory,
};

use super::{
    depth_history_repo::DepthHistoryRepository, earnings_history_repo::EarningsHistoryRepository,
    rune_pool_history_repo::RunePoolHistoryRepository, swaps_history_repo::SwapsHistoryRepository,
};

pub struct MongoDB {
    pub depth_history_repo: DepthHistoryRepository,
    pub earnings_history_repo: EarningsHistoryRepository,
    pub rune_pool_history_repo: RunePoolHistoryRepository,
    pub swaps_history_repo: SwapsHistoryRepository,
}

impl MongoDB {
    pub async fn init() -> Result<Self, &'static str> {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => return Err("Error loading the mongodb uri."),
        };
        let client = Some(Client::with_uri_str(uri).await.unwrap());

        let client = match client {
            Some(clt) => clt,
            None => return Err("Failed connecting to the mongodb client, check the mongouri."),
        };

        let db = client.database("rustmidgardapi");

        let depth_history_collection: Collection<DepthHistory> = db.collection("depth_history");
        let earnings_history_collection: Collection<EarningsHistory> =
            db.collection("earnings_collection");
        let swaps_history_collection: Collection<SwapsHistory> = db.collection("swaps_history");
        let rune_pool_collection: Collection<RunePoolHistory> = db.collection("rune_pool");

        let depth_history_repo = DepthHistoryRepository::init(depth_history_collection)
            .await
            .unwrap();
        let earnings_history_repo = EarningsHistoryRepository::init(earnings_history_collection)
            .await
            .unwrap();

        let swaps_history_repo = SwapsHistoryRepository::init(swaps_history_collection)
            .await
            .unwrap();

        let rune_pool_history_repo = RunePoolHistoryRepository::init(rune_pool_collection)
            .await
            .unwrap();

        Ok(MongoDB {
            depth_history_repo,
            earnings_history_repo,
            rune_pool_history_repo,
            swaps_history_repo,
        })
    }
}
