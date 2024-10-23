use actix_web::web;

use actix_web::{get, web::Data, HttpResponse};

use crate::models::earnings_history_model::{EarningsHistory, EarningsHistoryPool};
use crate::{
    models::earnings_history_model::EarningsHistoryResponse,
    repository::mongodb_repository::MongoDB,
};

#[get("/fetch-swaps")]
pub async fn fetch_earnings_history(db: Data<MongoDB>) -> HttpResponse {
    let mut start_time = String::from("1648771200");
    let count = 400;

    loop {
        let url = format!(
            "https://midgard.ninerealms.com/v2/history/swaps?interval=hour&count={}&from={}",
            count, start_time
        );

        let mut count = 1;

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<EarningsHistoryResponse>().await {
                Ok(resp) => {
                    start_time = resp.meta.end_time.clone();
                    for earnings_history in resp.intervals {
                        let mut earnings_history_2: Option<EarningsHistory> = None;
                        for pools in earnings_history.pools {
                            if pools.pools == "BTC.BTC" {
                                earnings_history_2 = Some(EarningsHistory {
                                    start_time: earnings_history.start_time,
                                    end_time: earnings_history.end_time,
                                    liquidity_fees: earnings_history.liquidity_fees,
                                    block_rewards: earnings_history.block_rewards,
                                    earnings: earnings_history.earnings,
                                    bonding_earnings: earnings_history.bonding_earnings,
                                    liquidity_earnings: earnings_history.liquidity_earnings,
                                    rune_price_usd: earnings_history.rune_price_usd,
                                    avg_node_count: earnings_history.avg_node_count,
                                    pools: vec![pools],
                                });

                                break;
                            }
                        }

                        if let Some(valid_earnings_history) = earnings_history_2 {
                            match db
                                .earnings_history_repo
                                .insert_earnings_history(&valid_earnings_history)
                                .await
                            {
                                Ok(_) => {
                                    println!("{}", count);
                                }
                                Err(_) => {
                                    eprintln!("Failed to insert earnings data into database");
                                    return HttpResponse::InternalServerError()
                                        .body("Failed to insert earnings data into database");
                                }
                            }
                        } else {
                            break;
                        }

                        count += 1;
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
    config;
    ()
}
