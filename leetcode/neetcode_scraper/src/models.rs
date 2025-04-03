use serde::{Deserialize, Serialize};

/// Represents a problem from NeetCode with its title and LeetCode link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    /// The title of the problem
    pub title: String,
    
    /// The URL to the problem on LeetCode
    pub leetcode_link: String,
}

/// Chrome for Testing API response structures
#[derive(Debug, Deserialize)]
pub struct ChromeForTestingVersions {
    #[serde(rename = "timestamp")]
    _timestamp: String,
    pub channels: ChannelsData,
}

#[derive(Debug, Deserialize)]
pub struct ChannelsData {
    #[serde(rename = "Stable")]
    pub stable: ChannelInfo,
    
    #[serde(rename = "Beta")]
    pub beta: ChannelInfo,
    
    #[serde(rename = "Dev")]
    pub dev: ChannelInfo,
    
    #[serde(rename = "Canary")]
    pub canary: ChannelInfo,
}

#[derive(Debug, Deserialize)]
pub struct ChannelInfo {
    pub version: String,
    pub revision: String,
    pub downloads: DownloadTypes,
}

#[derive(Debug, Deserialize)]
pub struct DownloadTypes {
    #[serde(rename = "chromedriver")]
    pub chromedriver: Vec<DownloadInfo>,
    
    #[serde(rename = "chrome")]
    pub chrome: Vec<DownloadInfo>,
    
    #[serde(rename = "chrome-headless-shell")]
    pub chrome_headless_shell: Vec<DownloadInfo>,
}

#[derive(Debug, Deserialize)]
pub struct DownloadInfo {
    pub platform: String,
    pub url: String,
}
