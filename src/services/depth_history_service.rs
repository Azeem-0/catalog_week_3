use std::vec;

use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};
use chrono::Utc;

use crate::{
    models::depth_history_model::DepthHistoryResponse,
    repository::mongodb_repository::MongoDB,
    utils::{query_parameters::QueryParameters, time_interval::TimeInterval},
};

#[get("/fetch-and-insert-depth")]
pub async fn fetch_and_insert_depth_history(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let params = query.into_inner();

    let mut from: f64 = params.from.clone().unwrap_or_else(|| 1648771200.0);
    let count = params.count.unwrap_or_else(|| 400.0);
    let interval = params.interval.unwrap_or_else(|| String::from("year"));
    let pool = params.pool.unwrap_or_else(|| String::from("BTC.BTC"));

    let mut depth_docs_count = 1;

    loop {
        let current_time = Utc::now().timestamp() as f64;

        if from >= current_time {
            println!("Start time has reached or exceeded the current time, breaking the loop.");
            break;
        }

        let url = format!(
            "https://midgard.ninerealms.com/v2/history/depths/{}?interval={}&count={}&from={}",
            pool, interval, count, from
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

#[get("")]
pub async fn depth_history_api(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let (from, count, interval, to, page, sort_by, pool) = query.process_query_parameters();

    println!("{} {} {} {} {} {}", from, count, to, pool, sort_by, page);

    let result = db
        .depth_history_repo
        .fetch_depth_history_data(from, to, count, interval, page, sort_by)
        .await
        .unwrap_or_else(|e| vec![]);

    HttpResponse::Ok().json(result)
}
pub fn init(config: &mut web::ServiceConfig) -> () {
    config
        .service(fetch_and_insert_depth_history)
        .service(depth_history_api);
    ()
}
