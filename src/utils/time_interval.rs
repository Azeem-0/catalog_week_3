use core::fmt;

#[derive(Debug)]
pub enum TimeInterval {
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

impl TimeInterval {
    pub fn as_seconds(&self) -> i64 {
        match self {
            TimeInterval::Hour => 3600,       // 1 hour
            TimeInterval::Day => 86400,       // 1 day
            TimeInterval::Week => 604800,     // 1 week
            TimeInterval::Month => 2678400,   // ~30 days
            TimeInterval::Quarter => 7948800, // ~3 months
            TimeInterval::Year => 31822400,   // 1 year
        }
    }

    // Add a method to convert from string to enum
    pub fn from_str(interval: &str) -> Option<Self> {
        match interval.to_lowercase().as_str() {
            "hour" => Some(TimeInterval::Hour),
            "day" => Some(TimeInterval::Day),
            "week" => Some(TimeInterval::Week),
            "month" => Some(TimeInterval::Month),
            "quarter" => Some(TimeInterval::Quarter),
            "year" => Some(TimeInterval::Year),
            _ => None, // Return None if the string doesn't match any interval
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            TimeInterval::Hour => "hour",
            TimeInterval::Day => "day",
            TimeInterval::Week => "week",
            TimeInterval::Month => "month",
            TimeInterval::Quarter => "quarter",
            TimeInterval::Year => "year",
        }
    }
}
