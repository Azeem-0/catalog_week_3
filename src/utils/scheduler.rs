use actix_web::web::Data;
use chrono::Utc;
use futures::{StreamExt, TryStreamExt};
use mongodb::{bson::doc, options::FindOptions};
use tokio::time::{interval, Duration};

use crate::{
    repository::mongodb_repository::MongoDB,
    services::{
        depth_history_service::{self},
        earnings_history_service, rune_pool_history_service, swaps_history_service,
    },
};

pub async fn get_last_end_time(db: &Data<MongoDB>) -> f64 {
    let options = FindOptions::builder()
        .sort(doc! { "endTime": -1 })
        .limit(1)
        .build();

    let mut result = db.depth_history_repo.col.find(None, options).await.unwrap();

    if let Some(doc) = result.try_next().await.unwrap() {
        return doc.end_time;
    }

    0.0
}

pub async fn run_cron_job(db: Data<MongoDB>) {
    let mut interval = interval(Duration::from_secs(3600));

    loop {
        interval.tick().await; // Waiting for the next tick.

        let from = get_last_end_time(&db).await;

        let interval = String::from("hour");

        println!("Running scheduled data fetch at {:?}", from);

        let depth_history_result = depth_history_service::fetch_and_update_depth_history(
            db.clone(),
            from,
            400.0,
            interval.to_string(),
            String::from("BTC.BTC"),
        )
        .await;

        let swap_history_result = swaps_history_service::fetch_and_update_swaps_history(
            &db,
            from,
            400.0,
            interval.to_string(),
            String::from("BTC.BTC"),
        )
        .await;

        let rune_pool_history_result =
            rune_pool_history_service::fetch_and_update_rune_pool_history(
                &db,
                from,
                400.0,
                interval.to_string(),
            )
            .await;

        let earnings_history_result = earnings_history_service::fetch_and_update_earnigns_history(
            &db,
            from,
            400.0,
            interval.to_string(),
        )
        .await;

        println!(
            "Jobs done : depth history - {}, swap history - {}, rune pool history - {} earnings history {}",
            depth_history_result,
            swap_history_result,
            rune_pool_history_result,
            earnings_history_result
        );

        println!("Cron job running");
    }
}
