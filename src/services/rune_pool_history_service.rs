use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};
use chrono::Utc;

use crate::{
    models::rune_pool_history_model::{RunePoolHistoryMeta, RunePoolHistoryResponse},
    repository::mongodb_repository::MongoDB,
    utils::query_parameters::QueryParameters,
};

pub async fn fetch_and_update_rune_pool_history(
    db: &Data<MongoDB>,
    from: f64,
    count: f64,
    interval: String,
) -> bool {
    let url = format!(
        "https://midgard.ninerealms.com/v2/history/runepool?interval={}&count={}&from={}",
        interval, count, from
    );

    match reqwest::get(&url).await {
        Ok(response) => match response.json::<RunePoolHistoryResponse>().await {
            Ok(resp) => {
                for rune_pool in resp.intervals {
                    let _ = db
                        .rune_pool_history_repo
                        .insert_rune_pool_history(&rune_pool)
                        .await;
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
#[get("/fetch-and-insert-rune-pool")]
pub async fn fetch_and_insert_rune_pool_history(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let params = query.into_inner();

    let mut from: f64 = params.from.clone().unwrap_or_else(|| 1648771200.0);
    let count = params.count.unwrap_or_else(|| 400.0);
    let interval = params.interval.unwrap_or_else(|| String::from("year"));

    let mut rune_pool_docs_count = 0;

    loop {
        let current_time = Utc::now().timestamp() as f64;

        if from >= current_time {
            println!("Start time has reached or exceeded the current time, breaking the loop.");
            break;
        }

        let url = format!(
            "https://midgard.ninerealms.com/v2/history/runepool?interval={}&count={}&from={}",
            interval, count, from
        );

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<RunePoolHistoryResponse>().await {
                Ok(resp) => {
                    from = resp.meta.end_time.clone();
                    for rune_pool in resp.intervals {
                        let _ = db
                            .rune_pool_history_repo
                            .insert_rune_pool_history(&rune_pool)
                            .await;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to deserialize response: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .body("Failed to parse rune pool data");
                }
            },
            Err(e) => {
                eprintln!("Failed to fetch data: {:?}", e);
                return HttpResponse::InternalServerError().body("Failed to fetch rune pool data");
            }
        }
        rune_pool_docs_count += 400;

        println!("inserted {} docs", rune_pool_docs_count);
    }

    HttpResponse::Ok()
        .body("Successfully fetched and inserted rune pool history data into database.")
}

#[utoipa::path(
    get,
    path = "/rune-pool-history",
    params(
        ("from" = Option<f64>, Query, description = "Start time for fetching data in Unix timestamp format. Defaults to `1648771200.0` if not provided."),
        ("count" = Option<i64>, Query, description = "Number of records to fetch. Defaults to `400.0` if not provided or if the provided value is out of range (must be > 0.0 and <= 400.0)."),
        ("interval" = Option<String>, Query, description = "Time interval for the data (e.g., day, week, month,quarter,year). Defaults to `year` if not provided."),
        ("to" = Option<f64>, Query, description = "End time for fetching data in Unix timestamp format. Defaults to current time if not provided."),
        ("page" = Option<i64>, Query, description = "Page number for pagination. Defaults to `1` if not provided."),
        ("sort_by" = Option<String>, Query, description = "Field by which to sort the results (e.g., timestamp, price). Defaults to `startTime` if not provided or if the field is not present in the model."),
    ),
    responses(
        (status = 200, description = "Successfully fetched rune pool history data.", body = Vec<RunePoolHistoryResponse>),
        (status = 404, description = "No rune pool history found for the provided parameters."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "RUNEPool total members and units History", 
    operation_id = "fetchRunePoolHistoryData" 
)]
#[get("")]
pub async fn rune_pool_history_api(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let (from, count, interval, to, page, sort_by, pool) = query.process_query_parameters();

    println!("{} {} {} {} {} {}", from, count, to, pool, sort_by, page);

    let intervals = db
        .rune_pool_history_repo
        .fetch_rune_pool_history_data(from, to, count, interval, page, sort_by)
        .await
        .unwrap_or_else(|_| vec![]);

    let start_record = intervals.first().unwrap();
    let end_record = intervals.last().unwrap();

    let meta = RunePoolHistoryMeta {
        start_time: start_record.start_time,
        end_time: end_record.end_time,
        start_units: start_record.units,
        start_count: start_record.count,
        end_units: end_record.units,
        end_count: end_record.count,
    };
    let response = RunePoolHistoryResponse { meta, intervals };

    HttpResponse::Ok().json(response)
}

pub fn init(config: &mut web::ServiceConfig) -> () {
    config
        .service(fetch_and_insert_rune_pool_history)
        .service(rune_pool_history_api);
    ()
}
