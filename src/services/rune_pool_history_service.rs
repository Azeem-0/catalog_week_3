use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};

use crate::{
    models::rune_pool_history_model::RunePoolHistoryResponse,
    repository::mongodb_repository::MongoDB,
};

#[get("/fetch-rune-pool")]
pub async fn fetch_rune_pool_history(db: Data<MongoDB>) -> HttpResponse {
    let mut start_time = String::from("1648771200");
    let count = 400;

    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/runepool?interval=hour&count={}&from={}",
            count, start_time
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
    }

    HttpResponse::Ok().body("Riya ekkada?")
}
pub fn init(config: &mut web::ServiceConfig) -> () {
    config.service(fetch_rune_pool_history);
    ()
}
