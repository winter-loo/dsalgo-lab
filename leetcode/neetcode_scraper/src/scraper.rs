use crate::error::{Error, Result};
use crate::models::Problem;
use crate::config::Config;
use fantoccini::{Client, ClientBuilder, Locator};
use std::time::Duration;
use std::fs::File;
use std::io::Write;
use serde_json;

/// NeetCode scraper for extracting problems and their LeetCode links
pub struct Scraper {
    /// Configuration for the scraper
    config: Config,
    
    /// WebDriver client
    client: Option<Client>,
}

impl Scraper {
    /// Create a new scraper with the given configuration
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: None,
        }
    }
    
    /// Connect to the WebDriver
    pub async fn connect(&mut self, port: u16) -> Result<()> {
        log::info!("Connecting to WebDriver on port {}", port);
        
        // Set up the WebDriver client
        let mut caps = serde_json::map::Map::new();
        let chrome_opts = serde_json::json!({
            "args": ["--headless", "--no-sandbox", "--disable-dev-shm-usage"],
        });
        caps.insert("goog:chromeOptions".to_string(), chrome_opts);

        // Connect to WebDriver
        let client = ClientBuilder::native()
            .capabilities(caps)
            .connect(&format!("http://localhost:{}", port))
            .await
            .map_err(Error::Connection)?;
            
        self.client = Some(client);
        log::info!("Connected to WebDriver successfully");
        
        Ok(())
    }
    
    /// Scrape problems from NeetCode
    pub async fn scrape(&mut self) -> Result<Vec<Problem>> {
        if self.client.is_none() {
            return Err(Error::ScrapingError("WebDriver client not connected".to_string()));
        }
        
        let client = self.client.as_mut().unwrap();
        
        log::info!("Navigating to {}", self.config.url);
        client.goto(&self.config.url).await?;
        
        // Wait for the page to load (wait for the table to appear)
        log::info!("Waiting for page to load...");
        client.wait().for_element(Locator::Css("app-pattern-table tbody")).await?;
        
        // Wait a bit more for JavaScript to fully render the content
        tokio::time::sleep(Duration::from_secs(3)).await;
        
        log::info!("Page loaded, extracting problems...");
        
        // Find all table bodies
        let table_bodies = client.find_all(Locator::Css("app-pattern-table tbody")).await?;
        log::info!("Found {} table bodies", table_bodies.len());
        
        let mut problems = Vec::new();
        
        // Iterate through each table body
        for (i, table_body) in table_bodies.iter().enumerate() {
            log::info!("Processing table body {}", i + 1);
            
            // Find all rows in the current table body
            let rows = table_body.find_all(Locator::Css("tr")).await?;
            log::info!("Found {} rows in table body {}", rows.len(), i + 1);
            
            // Iterate through each row
            for (j, row) in rows.iter().enumerate() {
                // Get the entire third column content
                let td_element = match row.find(Locator::Css("td:nth-child(3)")).await {
                    Ok(element) => element,
                    Err(_) => {
                        log::warn!("Could not find third column in row {}", j + 1);
                        continue; // Skip if element not found
                    }
                };
                
                // Get all links in the third column
                let links = td_element.find_all(Locator::Css("a")).await?;
                
                if links.len() < 2 {
                    log::warn!("Not enough links in the third column of row {} (found {})", j + 1, links.len());
                    continue;
                }
                
                // Extract the problem title from the first link
                let title_element = &links[0];
                
                // Extract the LeetCode link from the second link
                let link_element = &links[1];
                
                // Get the href attribute for the LeetCode link
                let leetcode_link = link_element.attr("href").await?.unwrap_or_default();
                
                // Get the title using innerText (more reliable)
                let title = match title_element.prop("innerText").await {
                    Ok(Some(inner_text)) if !inner_text.trim().is_empty() => inner_text.trim().to_string(),
                    _ => {
                        // Fallback to text() method if innerText fails or is empty
                        let text_title = title_element.text().await?;
                        if !text_title.trim().is_empty() {
                            text_title.trim().to_string()
                        } else {
                            // Debug output for empty titles
                            log::warn!("Empty title detected in row {}", j + 1);
                            if let Ok(html) = title_element.html(true).await {
                                log::debug!("Title element HTML: {}", html);
                            }
                            
                            log::warn!("Skipping problem with empty title");
                            continue;
                        }
                    }
                };
                
                // Add the problem to our collection
                problems.push(Problem {
                    title,
                    leetcode_link,
                });
            }
        }
        
        log::info!("Scraped {} problems", problems.len());
        
        // Save to file if configured
        if self.config.save_to_file && self.config.output_file.is_some() {
            self.save_to_file(&problems)?;
        }
        
        Ok(problems)
    }
    
    /// Save problems to a file
    fn save_to_file(&self, problems: &[Problem]) -> Result<()> {
        if let Some(output_file) = &self.config.output_file {
            log::info!("Saving problems to file: {:?}", output_file);
            
            let json = serde_json::to_string_pretty(problems)?;
            let mut file = File::create(output_file)?;
            file.write_all(json.as_bytes())?;
            
            log::info!("Saved {} problems to file", problems.len());
        }
        
        Ok(())
    }
    
    /// Close the WebDriver client
    pub async fn close(&mut self) -> Result<()> {
        if let Some(client) = self.client.take() {
            log::info!("Closing WebDriver client");
            client.close().await?;
            log::info!("WebDriver client closed");
        }
        
        Ok(())
    }
}

impl Drop for Scraper {
    fn drop(&mut self) {
        if self.client.is_some() {
            log::warn!("Scraper dropped without closing the WebDriver client");
        }
    }
}
