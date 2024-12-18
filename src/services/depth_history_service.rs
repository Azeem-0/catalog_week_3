use std::vec;

use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};
use chrono::Utc;

use crate::{
    models::depth_history_model::{DepthHistoryMeta, DepthHistoryResponse},
    repository::mongodb_repository::MongoDB,
    utils::query_parameters::QueryParameters,
};

pub async fn fetch_and_update_depth_history(
    db: Data<MongoDB>,
    from: f64,
    count: f64,
    interval: String,
    pool: String,
) -> bool {
    let url = format!(
        "https://midgard.ninerealms.com/v2/history/depths/{}?interval={}&count={}&from={}",
        pool, interval, count, from
    );

    match reqwest::get(&url).await {
        Ok(response) => match response.json::<DepthHistoryResponse>().await {
            Ok(resp) => {
                for depth_history in resp.intervals {
                    match db
                        .depth_history_repo
                        .insert_depth_history(&depth_history)
                        .await
                    {
                        Ok(_) => (),
                        Err(_) => {
                            eprintln!("Failed to insert depth history data into database");
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

#[get("/fetch-and-insert-depth")]
pub async fn fetch_and_insert_depth_history(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let (from, count, interval, _, _, _, pool) = query.process_query_parameters();

    let mut from = from;
    let mut depth_docs_count = 1;

    loop {
        let current_time = Utc::now().timestamp() as f64;

        if from >= current_time {
            println!("Start time has reached or exceeded the current time, breaking the loop.");
            break;
        }

        let url = format!(
            "https://midgard.ninerealms.com/v2/history/depths/{}?interval={}&count={}&from={}",
            pool,
            interval.to_str(),
            count,
            from
        );

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<DepthHistoryResponse>().await {
                Ok(resp) => {
                    from = resp.meta.end_time.clone();
                    for depth_history in resp.intervals {
                        match db
                            .depth_history_repo
                            .insert_depth_history(&depth_history)
                            .await
                        {
                            Ok(_) => (),
                            Err(_) => {
                                eprintln!("Failed to insert depth history data into database");
                                return HttpResponse::InternalServerError()
                                    .body("Failed to insert depth history data into database");
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

        depth_docs_count += 400;

        println!("inserted {} depth docs", depth_docs_count);
    }

    HttpResponse::Ok().body("Successfully fetched and inserted depth history data into database.")
}

#[utoipa::path(
    get,
    path = "/depth-history",
    params(
        ("from" = Option<f64>, Query, description = "Start time for fetching data in Unix timestamp format. Defaults to `1648771200.0` if not provided."),
        ("count" = Option<i64>, Query, description = "Number of records to fetch. Defaults to `1.0` if not provided or if the provided value is out of range (must be > 0.0 and <= 400.0)."),
        ("interval" = Option<String>, Query, description = "Time interval for the data (e.g., day, week, month,quarter,year). Defaults to `year` if not provided."),
        ("to" = Option<f64>, Query, description = "End time for fetching data in Unix timestamp format. Defaults to current time if not provided."),
        ("page" = Option<i64>, Query, description = "Page number for pagination. Defaults to `1` if not provided."),
        ("sort_by" = Option<String>, Query, description = "Field by which to sort the results (e.g., timestamp, price). Defaults to `startTime` if not provided or if the field is not present in the model."),
        ("pool" = Option<String>, Query, description = "Asset pool to fetch data from (e.g., BTC.BTC). Currently working only with BTC.BTC.")
    ),
    responses(
        (status = 200, description = "Successfully fetched depth history data", body = Vec<DepthHistoryResponse>),
        (status = 404, description = "No depth history found for the provided parameters"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Depth and Price History",
    operation_id = "fetchDepthHistoryData"
)]
#[get("")]
pub async fn depth_history_api(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let (from, count, interval, to, page, sort_by, pool) = query.process_query_parameters();

    println!("{} {} {} {} {} {}", from, count, to, pool, sort_by, page);

    let intervals = db
        .depth_history_repo
        .fetch_depth_history_data(from, to, count, interval, page, sort_by)
        .await
        .unwrap_or_else(|_| vec![]);

    if intervals.len() == 0 {
        return HttpResponse::Ok().body("No data available for the specified interval or the query parameters may be incorrectly specified.");
    } else {
        let start_record = intervals.first().unwrap();
        let end_record = intervals.last().unwrap();

        let meta = DepthHistoryMeta {
            start_time: start_record.start_time,
            end_time: end_record.end_time,
            price_shift_loss: end_record.asset_price - start_record.asset_price,
            luvi_increase: end_record.luvi - start_record.luvi,
            start_asset_depth: start_record.asset_depth,
            start_rune_depth: start_record.rune_depth,
            start_lp_units: start_record.liquidity_units,
            start_member_count: start_record.members_count,
            start_synth_units: start_record.synth_units,
            end_asset_depth: end_record.asset_depth,
            end_rune_depth: end_record.rune_depth,
            end_lp_units: end_record.liquidity_units,
            end_member_count: end_record.members_count,
            end_synth_units: end_record.synth_units,
        };

        let response = DepthHistoryResponse { meta, intervals };

        HttpResponse::Ok().json(response)
    }
}

pub fn init(config: &mut web::ServiceConfig) -> () {
    config
        .service(fetch_and_insert_depth_history)
        .service(depth_history_api);
    ()
}
