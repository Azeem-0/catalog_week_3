use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};
use chrono::Utc;

use crate::{
    models::{query_parameters::QueryParameters, swaps_history_model::SwapsHistoryResponse},
    repository::mongodb_repository::MongoDB,
};

#[get("/fetch-and-insert-swaps")]
pub async fn fetch_and_insert_swaps_history(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let params = query.into_inner();

    let mut start_time = params.from.clone().unwrap_or_else(|| 1648771200);
    let count = params.count.unwrap_or_else(|| 400);
    let interval = params.interval.unwrap_or_else(|| String::from("year"));
    let pool = params.pool.unwrap_or_else(|| String::from("BTC.BTC"));

    let mut swaps_docs_count = 0;

    loop {
        let current_time = Utc::now().timestamp() as i64;

        println!("{} {} ", start_time, current_time);

        if start_time >= current_time {
            println!("Start time has reached or exceeded the current time, breaking the loop.");
            break;
        }

        let url = format!(
            "https://midgard.ninerealms.com/v2/history/swaps?pool={}&interval={}&count={}&from={}",
            pool, interval, count, start_time
        );

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<SwapsHistoryResponse>().await {
                Ok(resp) => {
                    start_time = resp.meta.end_time.clone();
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
pub fn init(config: &mut web::ServiceConfig) -> () {
    config.service(fetch_and_insert_swaps_history);
    ()
}
