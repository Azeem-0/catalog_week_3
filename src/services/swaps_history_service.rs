use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};

use crate::{
    models::swaps_history_model::SwapsHistoryResponse, repository::mongodb_repository::MongoDB,
};

#[get("/fetch-swaps")]
pub async fn fetch_swaps_history(db: Data<MongoDB>) -> HttpResponse {
    let mut start_time = String::from("1648771200");
    let count = 400;

    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/swaps?interval=hour&count={}&from={}",
            count, start_time
        );

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<SwapsHistoryResponse>().await {
                Ok(resp) => {
                    start_time = resp.meta.end_time.clone();
                    for depth in resp.intervals {
                        db.swaps_history_repo.insert_swaps_history(&depth).await;
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

    HttpResponse::Ok().body("Namastee.. bossss.")
}
pub fn init(config: &mut web::ServiceConfig) -> () {
    config.service(fetch_swaps_history);
    ()
}
