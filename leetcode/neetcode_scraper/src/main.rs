use std::error::Error;
use neetcode_scraper::{ChromeDriver, Config, Scraper};
use log::{info, warn, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    env_logger::init();
    
    info!("Starting NeetCode 150 scraper...");
    
    // Parse command-line arguments to get configuration
    let config = match Config::from_args() {
        Ok(config) => config,
        Err(e) => {
            error!("Error parsing configuration: {}", e);
            return Err(e.into());
        }
    };
    
    // Set up ChromeDriver
    let mut chromedriver = ChromeDriver::new(config.clone());
    
    if let Err(e) = chromedriver.setup().await {
        error!("Failed to set up ChromeDriver: {}", e);
        return Err(e.into());
    }
    info!("ChromeDriver set up successfully");
    
    // Start ChromeDriver
    if let Err(e) = chromedriver.start() {
        error!("Failed to start ChromeDriver: {}", e);
        return Err(e.into());
    }
    info!("ChromeDriver started successfully on port {}", config.chromedriver_port);
    
    // Create and connect the scraper
    let mut scraper = Scraper::new(config.clone());
    
    if let Err(e) = scraper.connect(config.chromedriver_port).await {
        error!("Failed to connect to ChromeDriver: {}", e);
        // Make sure to stop ChromeDriver before returning
        let _ = chromedriver.stop();
        return Err(e.into());
    }
    info!("Connected to ChromeDriver successfully");
    
    // Run the scraper
    let problems = match scraper.scrape().await {
        Ok(problems) => problems,
        Err(e) => {
            error!("Error scraping NeetCode: {}", e);
            // Make sure to clean up before returning
            let _ = scraper.close().await;
            let _ = chromedriver.stop();
            return Err(e.into());
        }
    };
    
    info!("Successfully scraped {} problems", problems.len());
    
    // Print the results
    println!("\nSuccessfully scraped {} problems:\n", problems.len());
    
    for (i, problem) in problems.iter().enumerate() {
        println!("{:3}. {} - {}", i + 1, problem.title, problem.leetcode_link);
    }
    
    println!("\nDone!");
    
    // Clean up
    if let Err(e) = scraper.close().await {
        warn!("Error closing scraper: {}", e);
    }
    
    if let Err(e) = chromedriver.stop() {
        warn!("Error stopping ChromeDriver: {}", e);
    }
    
    Ok(())
}
