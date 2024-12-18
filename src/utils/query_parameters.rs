use serde::Deserialize;

use super::time_interval::TimeInterval;

#[derive(Deserialize, Clone)]
pub struct QueryParameters {
    pub from: Option<i64>,
    pub count: Option<i64>,
    pub interval: Option<String>,
    pub to: Option<i64>,
    pub pool: Option<String>,
    pub page: Option<i64>,
    pub sort_by: Option<String>,
}
impl QueryParameters {
    pub fn process_query_parameters(&self) -> (f64, f64, TimeInterval, f64, i64, String, String) {
        let count = match self.count {
            Some(value) if value > 0 && value <= 400 => value,
            _ => 1,
        };

        // Convert interval string to TimeInterval enum
        let interval = TimeInterval::from_str(self.interval.as_deref().unwrap_or("year"))
            .unwrap_or(TimeInterval::Year);

        let page = self.page.unwrap_or(1);
        let sort_by = self.sort_by.clone().unwrap_or(String::from("startTime"));
        let pool = self.pool.clone().unwrap_or_else(|| String::from("BTC.BTC"));

        let from: i64 = self.from.unwrap_or_else(|| {
            let effective_count = count as i64;
            let effective_interval = interval.as_seconds() as i64;

            let duration_seconds = effective_interval * effective_count;
            let current_time = 1729944000;

            println!(
                "{} {} {}",
                duration_seconds,
                current_time,
                (current_time - duration_seconds)
            );

            (current_time - duration_seconds) as i64
        });

        let to = self.to.unwrap_or_else(|| {
            let effective_count = count;
            let effective_interval = interval.as_seconds() as i64;

            // Calculate the maximum duration based on page, count, and interval
            let duration_needed = effective_count * effective_interval * page;

            println!("{} ", duration_needed + from + 1);
            // Limit `to` to the exact point where the number of records aligns with the count, page, and interval.

            if interval.to_str().eq("hour") || interval.to_str().eq("day") {
                duration_needed + from + 1 + duration_needed
            } else {
                duration_needed + from + 1
            }
        });

        (
            from as f64,
            count as f64,
            interval,
            to as f64,
            page,
            sort_by,
            pool,
        )
    }
}
