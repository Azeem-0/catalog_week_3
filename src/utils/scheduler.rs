use actix_web::web::Data;
use chrono::Utc;
use dotenv::dotenv;
use tokio::time::{interval, Duration};

use crate::{
    repository::mongodb_repository::MongoDB,
    services::{
        depth_history_service::{self},
        earnings_history_service, rune_pool_history_service, swaps_history_service,
    },
};

pub async fn run_cron_job(db: Data<MongoDB>) {
    let mut interval = interval(Duration::from_secs(3600));

    loop {
        interval.tick().await; // Waiting for the next tick.

        let curr_time = Utc::now().timestamp() as f64;
        let from = curr_time - 3600.0;
        let interval = String::from("hour");

        println!("Running scheduled data fetch at {:?}", from);

        let depth_history_result = depth_history_service::fetch_and_update_depth_history(
            db.clone(),
            from,
            1.0,
            interval.to_string(),
            String::from("BTC.BTC"),
        )
        .await;

        let swap_history_result = swaps_history_service::fetch_and_update_swaps_history(
            &db,
            from,
            1.0,
            interval.to_string(),
            String::from("BTC.BTC"),
        )
        .await;

        let rune_pool_history_result =
            rune_pool_history_service::fetch_and_update_rune_pool_history(
                &db,
                from,
                1.0,
                interval.to_string(),
            )
            .await;

        let earnings_history_result = earnings_history_service::fetch_and_update_earnigns_history(
            &db,
            from,
            1.0,
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
