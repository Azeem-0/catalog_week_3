use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};
use chrono::Utc;

use crate::{
    models::rune_pool_history_model::RunePoolHistoryResponse,
    repository::mongodb_repository::MongoDB, utils::query_parameters::QueryParameters,
};

pub async fn fetch_and_update_rune_pool_history(
    db: &Data<MongoDB>,
    from: f64,
    count: f64,
    interval: String,
) -> bool {
    let mut rune_pool_docs_count = 0;
    let mut from = from;

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

                        rune_pool_docs_count += 1;
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

#[get("")]
pub async fn rune_pool_history_api(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let (from, count, interval, to, page, sort_by, pool) = query.process_query_parameters();

    println!("{} {} {} {} {} {}", from, count, to, pool, sort_by, page);

    let result = db
        .rune_pool_history_repo
        .fetch_rune_pool_history_data(from, to, count, interval, page, sort_by)
        .await
        .unwrap_or_else(|e| vec![]);

    HttpResponse::Ok().json(result)
}

pub fn init(config: &mut web::ServiceConfig) -> () {
    config
        .service(fetch_and_insert_rune_pool_history)
        .service(rune_pool_history_api);
    ()
}
