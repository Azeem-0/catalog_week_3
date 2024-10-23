use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};

use crate::{
    models::{depth_history_model::DepthHistoryResponse, query_parameters::QueryParameters},
    repository::mongodb_repository::MongoDB,
};

#[get("/fetch-and-insert-depth")]
pub async fn fetch_and_insert_depth_history(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let params = query.into_inner();

    let mut start_time = params.from.clone().unwrap_or_else(|| 1648771200);
    let count = params.count.unwrap_or_else(|| 1);
    let interval = params.interval.unwrap_or_else(|| String::from("year"));
    let pool = params.pool.unwrap_or_else(|| String::from("BTC.BTC"));

    let mut depth_docs_count = 1;

    loop {
        let url = format!("https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=hour&count={}&from={}",count, start_time);

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<DepthHistoryResponse>().await {
                Ok(resp) => {
                    start_time = resp.meta.end_time.clone();
                    for depth in resp.intervals {
                        match db.depth_history_repo.insert_depth_history(&depth).await {
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
    }

    HttpResponse::Ok().body("Successfully fetched and inserted depth history data into database.")
}

pub fn init(config: &mut web::ServiceConfig) -> () {
    config.service(fetch_and_insert_depth_history);
    ()
}
