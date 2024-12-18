use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};
use chrono::Utc;

use crate::{
    models::swaps_history_model::{SwapsHistory, SwapsHistoryMeta, SwapsHistoryResponse},
    repository::mongodb_repository::MongoDB,
    utils::query_parameters::QueryParameters,
};

pub async fn fetch_and_update_swaps_history(
    db: &Data<MongoDB>,
    from: f64,
    count: f64,
    interval: String,
    pool: String,
) -> bool {
    let url = format!(
        "https://midgard.ninerealms.com/v2/history/swaps?pool={}&interval={}&count={}&from={}",
        pool, interval, count, from
    );

    match reqwest::get(&url).await {
        Ok(response) => match response.json::<SwapsHistoryResponse>().await {
            Ok(resp) => {
                for swaps_history in resp.intervals {
                    match db
                        .swaps_history_repo
                        .insert_swaps_history(&swaps_history)
                        .await
                    {
                        Ok(_) => (),
                        Err(_) => {
                            eprintln!("Failed to insert data into database");
                            return false;
                        }
                    }
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

    true
}

#[get("/fetch-and-insert-swaps")]
pub async fn fetch_and_insert_swaps_history(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let (from, count, interval, _, _, _, pool) = query.process_query_parameters();

    let mut swaps_docs_count = 0;
    let mut from = from;
    loop {
        let current_time = Utc::now().timestamp() as f64;

        if from >= current_time {
            println!("Start time has reached or exceeded the current time, breaking the loop.");
            break;
        }

        let url =
            format!(
            "https://midgard.ninerealms.com/v2/history/swaps?pool={}&interval={}&count={}&from={}",
            pool, interval.to_str(), count, from
        );

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<SwapsHistoryResponse>().await {
                Ok(resp) => {
                    from = resp.meta.end_time.clone();
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

pub async fn get_meta_information(
    start_record: &SwapsHistory,
    end_record: &SwapsHistory,
) -> SwapsHistoryMeta {
    SwapsHistoryMeta {
        start_time: start_record.start_time,
        end_time: end_record.end_time,
        to_asset_count: end_record.to_asset_count,
        to_rune_count: end_record.to_rune_count,
        to_trade_count: end_record.to_trade_count,
        from_trade_count: end_record.from_trade_count,
        synth_mint_count: end_record.synth_mint_count,
        synth_redeem_count: end_record.synth_redeem_count,
        total_count: end_record.total_count,
        to_asset_volume: end_record.to_asset_volume,
        to_rune_volume: end_record.to_rune_volume,
        to_trade_volume: end_record.to_trade_volume,
        from_trade_volume: end_record.from_trade_volume,
        synth_mint_volume: end_record.synth_mint_volume,
        synth_redeem_volume: end_record.synth_redeem_volume,
        total_volume: end_record.total_volume,
        to_asset_volume_usd: end_record.to_asset_volume_usd,
        to_rune_volume_usd: end_record.to_rune_volume_usd,
        to_trade_volume_usd: end_record.to_trade_volume_usd,
        from_trade_volume_usd: end_record.from_trade_volume_usd,
        synth_mint_volume_usd: end_record.synth_mint_volume_usd,
        synth_redeem_volume_usd: end_record.synth_redeem_volume_usd,
        total_volume_usd: end_record.total_volume_usd,
        to_asset_fees: end_record.to_asset_fees,
        to_rune_fees: end_record.to_rune_fees,
        to_trade_fees: end_record.to_trade_fees,
        from_trade_fees: end_record.from_trade_fees,
        synth_mint_fees: end_record.synth_mint_fees,
        synth_redeem_fees: end_record.synth_redeem_fees,
        total_fees: end_record.total_fees,
        to_asset_average_slip: end_record.to_asset_average_slip,
        to_rune_average_slip: end_record.to_rune_average_slip,
        to_trade_average_slip: end_record.to_trade_average_slip,
        from_trade_average_slip: end_record.from_trade_average_slip,
        synth_mint_average_slip: end_record.synth_mint_average_slip,
        synth_redeem_average_slip: end_record.synth_redeem_average_slip,
        average_slip: end_record.average_slip,
        rune_price_usd: end_record.rune_price_usd,
    }
}

#[utoipa::path(
    get,
    path = "/swaps-history",
    params(
        ("from" = Option<f64>, Query, description = "Start time for fetching data in Unix timestamp format. Defaults to `1648771200.0` if not provided."),
        ("count" = Option<i64>, Query, description = "Number of records to fetch. Defaults to `1.0` if not provided or if the provided value is out of range (must be > 0.0 and <= 400.0)."),
        ("interval" = Option<String>, Query, description = "Time interval for the data (e.g., day, week, month,quarter,year). Defaults to `year` if not provided."),
        ("to" = Option<f64>, Query, description = "End time for fetching data in Unix timestamp format. Defaults to current time if not provided."),
        ("page" = Option<i64>, Query, description = "Page number for pagination. Defaults to `1` if not provided."),
        ("sort_by" = Option<String>, Query, description = "Field by which to sort the results (e.g., timestamp, price). Defaults to `startTime` if not provided or if the field is not present in the model."),
        ("pool" = Option<String>, Query, description = "Asset pool to fetch data from (e.g., BTC.BTC). Currently working only with BTC.BTC.")
    ),
    responses(
        (status = 200, description = "Successfully fetched swaps history data.", body = Vec<SwapsHistoryResponse>),
        (status = 404, description = "No swaps history found for the provided parameters."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "Swaps History", 
    operation_id = "fetchSwapsHistoryData" 
)]
#[get("")]
pub async fn swaps_history_api(
    db: Data<MongoDB>,
    query: web::Query<QueryParameters>,
) -> HttpResponse {
    let (from, count, interval, to, page, sort_by, pool) = query.process_query_parameters();

    println!("{} {} {} {} {} {}", from, count, to, pool, sort_by, page);

    let intervals = db
        .swaps_history_repo
        .fetch_swaps_history_data(from, to, count, interval, page, sort_by)
        .await
        .unwrap_or_else(|_| vec![]);

    if intervals.len() == 0 {
        return HttpResponse::Ok().body("No data available for the specified interval or the query parameters may be incorrectly specified.");
    } else {
        let start_record = intervals.first().unwrap();
        let end_record = intervals.last().unwrap();

        let meta = get_meta_information(start_record, end_record).await;

        let response = SwapsHistoryResponse { meta, intervals };

        HttpResponse::Ok().json(response)
    }
}

pub fn init(config: &mut web::ServiceConfig) -> () {
    config
        .service(fetch_and_insert_swaps_history)
        .service(swaps_history_api);
    ()
}
