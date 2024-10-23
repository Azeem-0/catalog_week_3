use actix_web::web;

use actix_web::{get, web::Data, HttpResponse};
use chrono::Utc;

use crate::models::query_parameters::QueryParameters;
use crate::{
    models::earnings_history_model::EarningsHistoryResponse,
    repository::mongodb_repository::MongoDB,
};

#[get("/fetch-and-insert-earnings")]
pub async fn fetch_and_insert_earnings_history(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let params = query.into_inner();

    let mut start_time = params.from.clone().unwrap_or_else(|| 1648771200);
    let count = params.count.unwrap_or_else(|| 400);
    let interval = params.interval.unwrap_or_else(|| String::from("year"));

    let mut earnings_docs_count = 1;

    loop {
        let current_time = Utc::now().timestamp() as i64;

        println!("{} {} ", start_time, current_time);

        if start_time >= current_time {
            println!("Start time has reached or exceeded the current time, breaking the loop.");
            break;
        }

        let url = format!(
            "https://midgard.ninerealms.com/v2/history/earnings?interval={}&count={}&from={}",
            interval, count, start_time
        );

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<EarningsHistoryResponse>().await {
                Ok(resp) => {
                    start_time = resp.meta.end_time.clone();
                    for earnings_history in resp.intervals {
                        match db
                            .earnings_history_repo
                            .insert_earnings_history(&earnings_history)
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
                        // let mut earnings_history_2: Option<EarningsHistory> = None;
                        // for pools in earnings_history.pools {
                        //     if pools.pools == "BTC.BTC" {
                        //         earnings_history_2 = Some(EarningsHistory {
                        //             start_time: earnings_history.start_time,
                        //             end_time: earnings_history.end_time,
                        //             liquidity_fees: earnings_history.liquidity_fees,
                        //             block_rewards: earnings_history.block_rewards,
                        //             earnings: earnings_history.earnings,
                        //             bonding_earnings: earnings_history.bonding_earnings,
                        //             liquidity_earnings: earnings_history.liquidity_earnings,
                        //             rune_price_usd: earnings_history.rune_price_usd,
                        //             avg_node_count: earnings_history.avg_node_count,
                        //             pools: vec![pools],
                        //         });

                        //         break;
                        //     }
                        // }

                        // if let Some(valid_earnings_history) = earnings_history_2 {
                        //     match db
                        //         .earnings_history_repo
                        //         .insert_earnings_history(&valid_earnings_history)
                        //         .await
                        //     {
                        //         Ok(_) => {
                        //             println!("{}", count);
                        //         }
                        //         Err(_) => {
                        //             eprintln!("Failed to insert earnings data into database");
                        //             return HttpResponse::InternalServerError()
                        //                 .body("Failed to insert earnings data into database");
                        //         }
                        //     }
                        // } else {
                        //     break;
                        // }
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
        earnings_docs_count += 400;

        println!("inserted {} docs", earnings_docs_count);
    }

    HttpResponse::Ok().body("Successfully fetched and inserted swaps history data into database.")
}

pub fn init(config: &mut web::ServiceConfig) -> () {
    config.service(fetch_and_insert_earnings_history);
    ()
}
