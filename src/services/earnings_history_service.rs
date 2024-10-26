use core::sync;

use actix_web::web;

use actix_web::{get, web::Data, HttpResponse};
use chrono::Utc;
use mongodb::error::Error;

use crate::models::earnings_history_model::{EarningsHistory, EarningsHistoryPool};
use crate::utils::query_parameters::QueryParameters;
use crate::{
    models::earnings_history_model::EarningsHistoryResponse,
    repository::mongodb_repository::MongoDB,
};

pub async fn fetch_and_update_earnigns_history(
    db: &Data<MongoDB>,
    from: f64,
    count: f64,
    interval: String,
) -> bool {
    let url = format!(
        "https://midgard.ninerealms.com/v2/history/earnings?interval={}&count={}&from={}",
        interval, count, from
    );

    match reqwest::get(&url).await {
        Ok(response) => match response.json::<EarningsHistoryResponse>().await {
            Ok(resp) => {
                for earnings_history in resp.intervals {
                    let earnings_history_2 = EarningsHistory {
                        avg_node_count: earnings_history.avg_node_count,
                        block_rewards: earnings_history.avg_node_count,
                        bonding_earnings: earnings_history.bonding_earnings,
                        earnings: earnings_history.earnings,
                        end_time: earnings_history.end_time,
                        liquidity_earnings: earnings_history.liquidity_earnings,
                        liquidity_fees: earnings_history.liquidity_fees,
                        rune_price_usd: earnings_history.rune_price_usd,
                        start_time: earnings_history.start_time,
                        pools: None,
                    };

                    let earnings_history_id = match db
                        .earnings_history_repo
                        .insert_earnings_history(&earnings_history_2)
                        .await
                    {
                        Ok(result) => result.inserted_id.as_object_id().unwrap(),
                        Err(_) => {
                            eprintln!("Failed to insert earnings data into database");
                            return false;
                        }
                    };

                    for pool in earnings_history.pools.unwrap() {
                        let pool_with_reference = EarningsHistoryPool {
                            start_time: Some(earnings_history_2.start_time),
                            end_time: Some(earnings_history_2.end_time),
                            asset_liquidity_fees: pool.asset_liquidity_fees,
                            earnings: pool.earnings,
                            rewards: pool.rewards,
                            rune_liquidity_fees: pool.rune_liquidity_fees,
                            saver_earning: pool.saver_earning,
                            total_liquidity_fees_rune: pool.total_liquidity_fees_rune,
                            earnings_history: Some(earnings_history_id),
                            pool: pool.pool,
                        };

                        match db
                            .earnings_history_repo
                            .insert_earnings_history_pool(&pool_with_reference)
                            .await
                        {
                            Ok(_) => (),
                            Err(_) => {
                                eprintln!("Failed to insert pool data into database");
                                return false;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to deserialize response: {:?}", e);
                return false;
            }
        },
        Err(e) => {
            eprintln!("Failed to fetch data: {:?}", e);
            return false;
        }
    }

    true
}

#[get("/fetch-and-insert-earnings")]
pub async fn fetch_and_insert_earnings_history(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let params = query.into_inner();

    let mut from: f64 = params.from.clone().unwrap_or_else(|| 1648771200.0);
    let count = params.count.unwrap_or_else(|| 400.0);
    let interval = params.interval.unwrap_or_else(|| String::from("year"));

    let mut earnings_docs_count = 0;

    loop {
        let current_time = Utc::now().timestamp() as f64;

        if from >= current_time {
            println!("Start time has reached or exceeded the current time, breaking the loop.");
            break;
        }

        let url = format!(
            "https://midgard.ninerealms.com/v2/history/earnings?interval={}&count={}&from={}",
            interval, count, from
        );

        println!("{}", url);

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<EarningsHistoryResponse>().await {
                Ok(resp) => {
                    from = resp.meta.end_time.clone();

                    for earnings_history in resp.intervals {
                        let earnings_history_2 = EarningsHistory {
                            avg_node_count: earnings_history.avg_node_count,
                            block_rewards: earnings_history.avg_node_count,
                            bonding_earnings: earnings_history.bonding_earnings,
                            earnings: earnings_history.earnings,
                            end_time: earnings_history.end_time,
                            liquidity_earnings: earnings_history.liquidity_earnings,
                            liquidity_fees: earnings_history.liquidity_fees,
                            rune_price_usd: earnings_history.rune_price_usd,
                            start_time: earnings_history.start_time,
                            pools: None,
                        };

                        let earnings_history_id = match db
                            .earnings_history_repo
                            .insert_earnings_history(&earnings_history_2)
                            .await
                        {
                            Ok(result) => result.inserted_id.as_object_id().unwrap(),
                            Err(_) => {
                                eprintln!("Failed to insert earnings data into database");
                                return HttpResponse::InternalServerError()
                                    .body("Failed to insert earnings data into database");
                            }
                        };

                        for pool in earnings_history.pools.unwrap() {
                            let pool_with_reference = EarningsHistoryPool {
                                start_time: Some(earnings_history_2.start_time),
                                end_time: Some(earnings_history_2.end_time),
                                asset_liquidity_fees: pool.asset_liquidity_fees,
                                earnings: pool.earnings,
                                rewards: pool.rewards,
                                rune_liquidity_fees: pool.rune_liquidity_fees,
                                saver_earning: pool.saver_earning,
                                total_liquidity_fees_rune: pool.total_liquidity_fees_rune,
                                earnings_history: Some(earnings_history_id),
                                pool: pool.pool,
                            };

                            match db
                                .earnings_history_repo
                                .insert_earnings_history_pool(&pool_with_reference)
                                .await
                            {
                                Ok(_) => (),
                                Err(_) => {
                                    eprintln!("Failed to insert pool data into database");
                                    return HttpResponse::InternalServerError()
                                        .body("Failed to insert pool data into database");
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to deserialize response: {:?}", e);
                    return HttpResponse::InternalServerError().body("Failed to parse data");
                }
            },
            Err(e) => {
                eprintln!("Failed to fetch data: {:?}", e);
                return HttpResponse::InternalServerError().body("Failed to fetch data");
            }
        }
        earnings_docs_count += 400;

        println!("inserted {} docs", earnings_docs_count);
    }

    HttpResponse::Ok().body("Successfully fetched and inserted swaps history data into database.")
}

#[utoipa::path(
    get,
    path = "/earnings-history",
    params(
        ("from" = Option<f64>, Query, description = "Start time for fetching earnings data in Unix timestamp format. Default: 1648771200.0"),
        ("count" = Option<i64>, Query, description = "Number of records to fetch. Default: 400"),
        ("interval" = Option<String>, Query, description = "Time interval for the data (e.g., day, week, month, year). Default: 'year'"),
        ("to" = Option<f64>, Query, description = "End time for fetching earnings data in Unix timestamp format. Default: 1729666800.0"),
        ("page" = Option<i64>, Query, description = "Page number for pagination. Default: 1"),
        ("sort_by" = Option<String>, Query, description = "Field by which to sort the results (e.g., start_time, earnings). Default: 'start_time'"),
        ("pool" = Option<String>, Query, description = "Asset pool to fetch data from (e.g., BTC.BTC). Default: 'BTC.BTC'")
    ),
    responses(
        (status = 200, description = "Successfully fetched earnings history data", body = Vec<EarningsHistory>),
        (status = 404, description = "No earnings history found for the provided parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Earnings History",
    operation_id = "fetchEarningsHistoryData"
)]
#[get("")]
pub async fn earnings_history_api(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let (from, count, interval, to, page, sort_by, pool) = query.process_query_parameters();

    println!("{} {} {} {} {} {}", from, count, to, pool, sort_by, page);

    let result = db
        .earnings_history_repo
        .fetch_earnings_history_data(from, to, count, interval, page, sort_by, pool)
        .await
        .unwrap_or_else(|_| vec![]);

    HttpResponse::Ok().json(result)
}

pub fn init(config: &mut web::ServiceConfig) -> () {
    config
        .service(fetch_and_insert_earnings_history)
        .service(earnings_history_api);
    ()
}
