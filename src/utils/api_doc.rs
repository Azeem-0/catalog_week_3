use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
        paths(
            crate::services::depth_history_service::depth_history_api,
            crate::services::earnings_history_service::earnings_history_api,
            crate::services::swaps_history_service::swaps_history_api,
            crate::services::rune_pool_history_service::rune_pool_history_api,
        ),
        components(schemas(
            crate::models::depth_history_model::DepthHistory,
            crate::models::depth_history_model::DepthHistoryResponse,
            crate::models::swaps_history_model::SwapsHistory,
            crate::models::swaps_history_model::SwapsHistoryResponse,
            crate::models::earnings_history_model::EarningsHistory,
            crate::models::earnings_history_model::EarningsHistoryPool,
            crate::models::earnings_history_model::EarningsHistoryResponse,
            crate::models::rune_pool_history_model::RunePoolHistory,
            crate::models::rune_pool_history_model::RunePoolHistoryResponse,
        )),
        tags(
            (name = "Depth and Price History", description = "Returns the asset and rune depths and price. The values report the state at the end of each interval."),
            (name = "Earnings History", description = "Returns earnings data for the specified interval."),
            (name = "Swaps History", description = "Returns swap count, volume, fees, slip in specified interval. If pool is not specified returns for all pools"),
            (name = "RUNEPool total members and units History", description = "Returns RUNEPool members and units. The values report the state at the end of each interval."),
        )
    )]
pub struct ApiDoc;
