use crate::error::{Error, Result};
use std::path::PathBuf;

/// Configuration for the NeetCode scraper
#[derive(Debug, Clone)]
pub struct Config {
    /// The URL to scrape
    pub url: String,
    
    /// The port to use for ChromeDriver
    pub chromedriver_port: u16,
    
    /// The directory to store ChromeDriver
    pub bin_dir: PathBuf,
    
    /// Whether to save the results to a file
    pub save_to_file: bool,
    
    /// The file to save results to (if save_to_file is true)
    pub output_file: Option<PathBuf>,
    
    /// Whether to force download a new ChromeDriver even if one exists
    pub force_download: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: "https://neetcode.io/practice?tab=neetcode150".to_string(),
            chromedriver_port: 9515,
            bin_dir: PathBuf::from("bin"),
            save_to_file: false,
            output_file: None,
            force_download: false,
        }
    }
}

impl Config {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Parse command-line arguments to create a configuration
    pub fn from_args() -> Result<Self> {
        let mut config = Self::default();
        
        // Get command-line arguments
        let args: Vec<String> = std::env::args().collect();
        
        // Parse arguments
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--url" | "-u" => {
                    if i + 1 < args.len() {
                        config.url = args[i + 1].clone();
                        i += 1;
                    } else {
                        return Err(Error::ConfigError("Missing URL value".to_string()));
                    }
                },
                "--port" | "-p" => {
                    if i + 1 < args.len() {
                        config.chromedriver_port = args[i + 1].parse().map_err(|_| {
                            Error::ConfigError(format!("Invalid port: {}", args[i + 1]))
                        })?;
                        i += 1;
                    } else {
                        return Err(Error::ConfigError("Missing port value".to_string()));
                    }
                },
                "--bin-dir" | "-b" => {
                    if i + 1 < args.len() {
                        config.bin_dir = PathBuf::from(&args[i + 1]);
                        i += 1;
                    } else {
                        return Err(Error::ConfigError("Missing bin directory value".to_string()));
                    }
                },
                "--output" | "-o" => {
                    if i + 1 < args.len() {
                        config.save_to_file = true;
                        config.output_file = Some(PathBuf::from(&args[i + 1]));
                        i += 1;
                    } else {
                        return Err(Error::ConfigError("Missing output file value".to_string()));
                    }
                },
                "--force-download" | "-f" => {
                    config.force_download = true;
                },
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                },
                _ => {
                    return Err(Error::ConfigError(format!("Unknown argument: {}", args[i])));
                }
            }
            i += 1;
        }
        
        // Check for environment variables
        if let Ok(url) = std::env::var("NEETCODE_URL") {
            config.url = url;
        }
        
        if let Ok(port) = std::env::var("CHROMEDRIVER_PORT") {
            if let Ok(port) = port.parse() {
                config.chromedriver_port = port;
            }
        }
        
        if let Ok(bin_dir) = std::env::var("CHROMEDRIVER_BIN_DIR") {
            config.bin_dir = PathBuf::from(bin_dir);
        }
        
        if let Ok(output_file) = std::env::var("OUTPUT_FILE") {
            config.save_to_file = true;
            config.output_file = Some(PathBuf::from(output_file));
        }
        
        if let Ok(force_download) = std::env::var("FORCE_DOWNLOAD") {
            config.force_download = force_download.to_lowercase() == "true";
        }
        
        Ok(config)
    }
}

/// Print help information
fn print_help() {
    println!("NeetCode Scraper - A tool to scrape problems from NeetCode");
    println!();
    println!("USAGE:");
    println!("    neetcode_scraper [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -u, --url <URL>                 The URL to scrape (default: https://neetcode.io/practice?tab=neetcode150)");
    println!("    -p, --port <PORT>               The port to use for ChromeDriver (default: 9515)");
    println!("    -b, --bin-dir <DIR>             The directory to store ChromeDriver (default: bin)");
    println!("    -o, --output <FILE>             Save results to a file");
    println!("    -f, --force-download            Force download a new ChromeDriver even if one exists");
    println!("    -h, --help                      Print this help information");
    println!();
    println!("ENVIRONMENT VARIABLES:");
    println!("    NEETCODE_URL                    The URL to scrape");
    println!("    CHROMEDRIVER_PORT               The port to use for ChromeDriver");
    println!("    CHROMEDRIVER_BIN_DIR            The directory to store ChromeDriver");
    println!("    OUTPUT_FILE                     Save results to a file");
    println!("    FORCE_DOWNLOAD                  Force download a new ChromeDriver even if one exists");
}
