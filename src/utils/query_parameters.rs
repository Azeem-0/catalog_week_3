use chrono::Utc;
use serde::Deserialize;

use super::time_interval::TimeInterval;

#[derive(Deserialize, Clone)]
pub struct QueryParameters {
    pub from: Option<f64>,
    pub count: Option<f64>,
    pub interval: Option<String>,
    pub to: Option<f64>,
    pub pool: Option<String>,
    pub page: Option<i64>,
    pub sort_by: Option<String>,
}
impl QueryParameters {
    pub fn process_query_parameters(&self) -> (f64, f64, TimeInterval, f64, i64, String, String) {
        let count = match self.count {
            Some(value) if value > 0.0 && value <= 400.0 => value,
            _ => 400.0,
        };

        // Convert interval string to TimeInterval enum
        let interval = TimeInterval::from_str(self.interval.as_deref().unwrap_or("year"))
            .unwrap_or(TimeInterval::Year);

        let page = self.page.unwrap_or(1);
        let sort_by = self.sort_by.clone().unwrap_or(String::from("startTime"));
        let to = self.to.unwrap_or_else(|| Utc::now().timestamp() as f64);
        let pool = self.pool.clone().unwrap_or_else(|| String::from("BTC.BTC"));

        let from = self.from.unwrap_or_else(|| {
            if self.count.is_none() && self.interval.is_none() {
                1648771200.0
            } else {
                let effective_count = count;
                let effective_interval = interval.as_seconds() as f64;
                let duration_seconds = effective_interval * effective_count;
                let current_time = Utc::now().timestamp() as f64;
                (current_time - duration_seconds).min(1648771200.0)
            }
        });

        (from, count, interval, to, page, sort_by, pool)
    }
}
