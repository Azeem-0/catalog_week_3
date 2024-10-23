use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};
use chrono::Utc;

use crate::{
    models::{query_parameters::QueryParameters, rune_pool_history_model::RunePoolHistoryResponse},
    repository::mongodb_repository::MongoDB,
};

#[get("/fetch-and-insert-rune-pool")]
pub async fn fetch_and_insert_rune_pool_history(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let params = query.into_inner();

    let mut start_time = params.from.clone().unwrap_or_else(|| 1648771200);
    let count = params.count.unwrap_or_else(|| 400);
    let interval = params.interval.unwrap_or_else(|| String::from("year"));

    let mut rune_pool_docs_count = 0;

    loop {
        let current_time = Utc::now().timestamp() as i64;

        if start_time >= current_time {
            println!("Start time has reached or exceeded the current time, breaking the loop.");
            break;
        }

        let url = format!(
            "https://midgard.ninerealms.com/v2/history/runepool?interval={}&count={}&from={}",
            interval, count, start_time
        );

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<RunePoolHistoryResponse>().await {
                Ok(resp) => {
                    start_time = resp.meta.end_time.clone();
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
pub fn init(config: &mut web::ServiceConfig) -> () {
    config.service(fetch_and_insert_rune_pool_history);
    ()
}
