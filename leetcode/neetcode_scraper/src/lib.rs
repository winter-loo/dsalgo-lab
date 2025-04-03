//! # NeetCode Scraper
//! 
//! A Rust library for scraping problem information from NeetCode.
//! 
//! This library provides functionality to:
//! - Detect Chrome version
//! - Download and manage ChromeDriver
//! - Scrape problems from NeetCode
//! - Save results to a file

pub mod chromedriver;
pub mod config;
pub mod error;
pub mod models;
pub mod scraper;
#[cfg(test)]
mod tests;

// Re-export commonly used types
pub use chromedriver::ChromeDriver;
pub use config::Config;
pub use error::{Error, Result};
pub use models::Problem;
pub use scraper::Scraper;
