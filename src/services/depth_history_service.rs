use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};

use futures::stream::repeat_with;
use reqwest::{dns::Resolve, Client, Response};
use serde_json::Result as SerdeResult;
use serde_json::{from_str, Value};

use crate::{
    models::depth_history_model::DepthHistoryResponse, repository::mongodb_repository::MongoDB,
};

#[get("/fetch-depth")]
pub async fn fetch_depth_history(db: Data<MongoDB>) -> HttpResponse {
    let mut start_time = String::from("1650387600");
    let count = 400;

    loop {
        let url = format!("https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=hour&count={}&from={}",count, start_time);

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<DepthHistoryResponse>().await {
                Ok(resp) => {
                    start_time = resp.meta.end_time.clone();
                    for depth in resp.intervals {
                        db.depth_history_repo.insert_depth_history(&depth).await;
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
    }

    HttpResponse::Ok().body("welcome")
}

#[get("/delete")]
pub async fn temp_delete(db: Data<MongoDB>) -> HttpResponse {
    db.depth_history_repo.temp_delete();
    HttpResponse::Ok().body("Deleted")
}
pub fn init(config: &mut web::ServiceConfig) -> () {
    config.service(fetch_depth_history).service(temp_delete);
    ()
}
