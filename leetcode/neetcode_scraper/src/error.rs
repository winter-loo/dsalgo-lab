use thiserror::Error;
use std::io;

/// Custom error types for the NeetCode scraper
#[derive(Error, Debug)]
pub enum Error {
    #[error("WebDriver error: {0}")]
    WebDriver(#[from] fantoccini::error::CmdError),
    
    #[error("WebDriver connection error: {0}")]
    Connection(#[from] fantoccini::error::NewSessionError),
    
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("ZIP extraction error: {0}")]
    ZipError(#[from] zip::result::ZipError),
    
    #[error("Chrome not found: {0}")]
    ChromeNotFound(String),
    
    #[error("ChromeDriver version not found: {0}")]
    ChromeDriverVersionNotFound(String),
    
    #[error("ChromeDriver download failed: {0}")]
    DownloadError(String),
    
    #[error("ChromeDriver start error: {0}")]
    StartError(String),
    
    #[error("Scraping error: {0}")]
    ScrapingError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Result type for the NeetCode scraper
pub type Result<T> = std::result::Result<T, Error>;
