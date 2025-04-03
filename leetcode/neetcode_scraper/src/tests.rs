//! Tests for the NeetCode scraper

#[cfg(test)]
mod tests {
    use crate::{Config, ChromeDriver, Scraper, Result};
    use std::path::PathBuf;
    
    /// Test that the default configuration is created correctly
    #[test]
    fn test_default_config() {
        let config = Config::default();
        
        assert_eq!(config.url, "https://neetcode.io/practice?tab=neetcode150");
        assert_eq!(config.chromedriver_port, 9515);
        assert_eq!(config.bin_dir, PathBuf::from("bin"));
        assert_eq!(config.save_to_file, false);
        assert_eq!(config.output_file, None);
        assert_eq!(config.force_download, false);
    }
    
    /// Test that the ChromeDriver executable name is correct for the current platform
    #[test]
    fn test_chromedriver_executable_name() {
        let name = ChromeDriver::get_executable_name();
        
        #[cfg(target_os = "windows")]
        assert_eq!(name, "chromedriver.exe");
        
        #[cfg(not(target_os = "windows"))]
        assert_eq!(name, "chromedriver");
    }
    
    /// Test that a ChromeDriver instance can be created with a config
    #[test]
    fn test_chromedriver_creation() {
        let config = Config::default();
        let chromedriver = ChromeDriver::new(config.clone());
        
        assert_eq!(chromedriver.path, config.bin_dir.join(ChromeDriver::get_executable_name()));
    }
    
    /// Test that a Scraper instance can be created with a config
    #[test]
    fn test_scraper_creation() {
        let config = Config::default();
        let _scraper = Scraper::new(config.clone());
        
        // Just verify it doesn't panic
        assert!(true);
    }
    
    /// Integration test for the ChromeDriver setup
    /// This test is ignored by default because it requires internet access
    /// and will download files
    #[test]
    #[ignore]
    fn test_chromedriver_setup() -> Result<()> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        rt.block_on(async {
            let config = Config {
                force_download: true,
                ..Config::default()
            };
            
            let mut chromedriver = ChromeDriver::new(config);
            chromedriver.setup().await?;
            
            assert!(chromedriver.exists());
            
            Ok(())
        })
    }
    
    /// Integration test for the scraper
    /// This test is ignored by default because it requires internet access
    /// and will start a ChromeDriver process
    #[test]
    #[ignore]
    fn test_scraper() -> Result<()> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        rt.block_on(async {
            let config = Config::default();
            
            // Set up ChromeDriver
            let mut chromedriver = ChromeDriver::new(config.clone());
            chromedriver.setup().await?;
            chromedriver.start()?;
            
            // Create and connect the scraper
            let mut scraper = Scraper::new(config);
            scraper.connect(9515).await?;
            
            // Run the scraper
            let problems = scraper.scrape().await?;
            
            // Verify we got some problems
            assert!(!problems.is_empty());
            
            // Clean up
            scraper.close().await?;
            chromedriver.stop()?;
            
            Ok(())
        })
    }
}
