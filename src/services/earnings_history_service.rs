use actix_web::web;

use crate::models::earnings_history_model::{
    EarningsHistory, EarningsHistoryMeta, EarningsHistoryPool,
};
use crate::utils::query_parameters::QueryParameters;
use crate::{
    models::earnings_history_model::EarningsHistoryResponse,
    repository::mongodb_repository::MongoDB,
};
use actix_web::{get, web::Data, HttpResponse};
use chrono::Utc;

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
                    match db
                        .earnings_history_repo
                        .insert_earnings_history(&earnings_history)
                        .await
                    {
                        Ok(_) => (),
                        Err(_) => {
                            eprintln!("Failed to insert earnings data into database");
                            return false;
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
    let (from, count, interval, to, page, sort_by, pool) = query.process_query_parameters();

    let mut earnings_docs_count = 0;
    let mut from = from;

    loop {
        let current_time = Utc::now().timestamp() as f64;

        if from >= current_time {
            println!("Start time has reached or exceeded the current time, breaking the loop.");
            break;
        }

        let url = format!(
            "https://midgard.ninerealms.com/v2/history/earnings?interval={}&count={}&from={}",
            interval.to_str(),
            count,
            from
        );

        println!("{}", url);

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<EarningsHistoryResponse>().await {
                Ok(resp) => {
                    from = resp.meta.end_time.clone();
                    for earnings_history in resp.intervals {
                        match db
                            .earnings_history_repo
                            .insert_earnings_history(&earnings_history)
                            .await
                        {
                            Ok(_) => (),
                            Err(_) => {
                                eprintln!("Failed to insert earnings data into database");
                                return HttpResponse::InternalServerError()
                                    .body("Failed to insert earnings data into database");
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
        ("from" = Option<f64>, Query, description = "Start time for fetching data in Unix timestamp format. Defaults to `1648771200.0` if not provided."),
        ("count" = Option<i64>, Query, description = "Number of records to fetch. Defaults to `400.0` if not provided or if the provided value is out of range (must be > 0.0 and <= 400.0)."),
        ("interval" = Option<String>, Query, description = "Time interval for the data (e.g., day, week, month,quarter,year). Defaults to `year` if not provided."),
        ("to" = Option<f64>, Query, description = "End time for fetching data in Unix timestamp format. Defaults to current time if not provided."),
        ("page" = Option<i64>, Query, description = "Page number for pagination. Defaults to `1` if not provided."),
        ("sort_by" = Option<String>, Query, description = "Field by which to sort the results (e.g., timestamp, price). Defaults to `startTime` if not provided or if the field is not present in the model."),
    ),
    responses(
        (status = 200, description = "Successfully fetched earnings history data", body = Vec<EarningsHistoryResponse>),
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

    println!(
        "{} {} {} {} {} {} {:?}",
        from, count, to, pool, sort_by, page, interval
    );

    let intervals = db
        .earnings_history_repo
        .fetch_earnings_history_data(from, to, count, interval, page, sort_by, pool)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Error occured {}", err);
            vec![]
        });

    if intervals.len() == 0 {
        return HttpResponse::Ok().body("No data available for the specified interval or query parameters may be incorrectly specified.");
    } else {
        let start_record = intervals.first().unwrap();
        let end_record = intervals.last().unwrap();

        let meta = EarningsHistoryMeta {
            start_time: start_record.start_time,
            end_time: end_record.end_time,
            liquidity_fees: end_record.liquidity_fees,
            block_rewards: end_record.block_rewards,
            earnings: end_record.earnings,
            bonding_earnings: end_record.bonding_earnings,
            liquidity_earnings: end_record.liquidity_earnings,
            avg_node_count: end_record.avg_node_count,
            rune_price_usd: end_record.rune_price_usd,
        };

        let response = EarningsHistoryResponse { intervals, meta };

        HttpResponse::Ok().json(response)
    }
}

pub fn init(config: &mut web::ServiceConfig) -> () {
    config
        .service(fetch_and_insert_earnings_history)
        .service(earnings_history_api);
    ()
}
