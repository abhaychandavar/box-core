use once_cell::sync::Lazy;
use std::env;

// pub static ENVIRONMENT: Lazy<String> = Lazy::new(|| {
//     env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string())
// });

pub static PORT: Lazy<String> = Lazy::new(|| {
    env::var("PORT").unwrap_or_else(|_| "8080".to_string())
});

pub static LOG_LEVEL: Lazy<String> = Lazy::new(|| {
    env::var("RUST_LOG").unwrap_or_else(|_| "None".to_string())
});

pub static PING_INTERVAL_SECS: Lazy<u64> = Lazy::new(|| {
    env::var("PING_INTERVAL_SECS")
        .unwrap_or_else(|_| "10".to_string())  // Default to "10" if env var is not found
        .parse::<u64>()
        .unwrap_or(10)  // Default to 10 if parsing fails
});

pub static PING_ELAPSED_SECS: Lazy<u64> = Lazy::new(|| {
    env::var("PING_ELAPSED_SECS")
    .unwrap_or_else(|_| "10".to_string())
    .parse::<u64>()
    .unwrap_or(10)
});

