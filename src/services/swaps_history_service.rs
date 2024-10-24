use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};
use chrono::Utc;

use crate::{
    models::swaps_history_model::SwapsHistoryResponse,
    repository::mongodb_repository::MongoDB,
    utils::{query_parameters::QueryParameters, time_interval::TimeInterval},
};

pub async fn fetch_and_update_swaps_history(
    db: &Data<MongoDB>,
    from: f64,
    count: f64,
    interval: String,
    pool: String,
) -> bool {
    let mut swaps_docs_count = 0;
    let mut from = from;

    loop {
        let current_time = Utc::now().timestamp() as f64;

        if from >= current_time {
            println!("Start time has reached or exceeded the current time, breaking the loop.");
            break;
        }

        let url = format!(
            "https://midgard.ninerealms.com/v2/history/swaps?pool={}&interval={}&count={}&from={}",
            pool, interval, count, from
        );

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<SwapsHistoryResponse>().await {
                Ok(resp) => {
                    from = resp.meta.end_time.clone();
                    for swaps_history in resp.intervals {
                        match db
                            .swaps_history_repo
                            .insert_swaps_history(&swaps_history)
                            .await
                        {
                            Ok(_) => (),
                            Err(_) => {
                                eprintln!("Failed to insert data into database");
                                return false;
                            }
                        }
                        swaps_docs_count += 1;
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
    }

    true
}

#[get("/fetch-and-insert-swaps")]
pub async fn fetch_and_insert_swaps_history(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let params = query.into_inner();

    let mut from: f64 = params.from.clone().unwrap_or_else(|| 1648771200.0);
    let count = params.count.unwrap_or_else(|| 400.0);
    let interval = params.interval.unwrap_or_else(|| String::from("year"));
    let pool = params.pool.unwrap_or_else(|| String::from("BTC.BTC"));

    let mut swaps_docs_count = 0;

    loop {
        let current_time = Utc::now().timestamp() as f64;

        if from >= current_time {
            println!("Start time has reached or exceeded the current time, breaking the loop.");
            break;
        }

        let url = format!(
            "https://midgard.ninerealms.com/v2/history/swaps?pool={}&interval={}&count={}&from={}",
            pool, interval, count, from
        );

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<SwapsHistoryResponse>().await {
                Ok(resp) => {
                    from = resp.meta.end_time.clone();
                    for swaps_history in resp.intervals {
                        match db
                            .swaps_history_repo
                            .insert_swaps_history(&swaps_history)
                            .await
                        {
                            Ok(_) => (),
                            Err(_) => {
                                eprintln!("Failed to insert data into database");
                                return HttpResponse::InternalServerError()
                                    .body("Failed to insert data into database");
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
        swaps_docs_count += 400;

        println!("inserted {} docs", swaps_docs_count);
    }

    HttpResponse::Ok().body("Successfully fetched and inserted swaps history data into database.")
}

#[get("/BTC.BTC")]
pub async fn swaps_history_api(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let (from, count, interval, to, page, sort_by, pool) = query.process_query_parameters();

    println!("{} {} {} {} {} {}", from, count, to, pool, sort_by, page);

    let result = db
        .swaps_history_repo
        .fetch_swaps_history_data(from, to, count, interval, page, sort_by)
        .await
        .unwrap_or_else(|e| vec![]);

    HttpResponse::Ok().json(result)
}

pub fn init(config: &mut web::ServiceConfig) -> () {
    config
        .service(fetch_and_insert_swaps_history)
        .service(swaps_history_api);
    ()
}
