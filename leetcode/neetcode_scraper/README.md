# NeetCode 150 Scraper

A modular, production-quality Rust application that scrapes the NeetCode 150 problems list from https://neetcode.io/practice?tab=neetcode150 and outputs a list of problem titles and their corresponding LeetCode links.

## Features

- Automatic ChromeDriver detection and installation
- Configurable via command-line arguments and environment variables
- Robust error handling
- Structured logging
- Modular code organization
- Option to save results to a JSON file

## Prerequisites

- Rust and Cargo
- Google Chrome (the scraper will automatically download a compatible ChromeDriver)

## Setup

Install Rust dependencies:
```
cargo build
```

## Usage

Run the scraper with default settings:
```
cargo run
```

Or with custom options:
```
cargo run -- --url "https://neetcode.io/practice?tab=neetcode150" --port 9516 --output problems.json
```

### Command-line Options

- `--url`, `-u`: The URL to scrape (default: https://neetcode.io/practice?tab=neetcode150)
- `--port`, `-p`: The port to use for ChromeDriver (default: 9515)
- `--bin-dir`, `-b`: The directory to store ChromeDriver (default: ./bin)
- `--output`, `-o`: The file to save results to (if specified, results will be saved as JSON)
- `--force-download`, `-f`: Force download a new ChromeDriver even if one exists
- `--help`, `-h`: Show help information

### Environment Variables

You can also configure the scraper using environment variables:

- `NEETCODE_URL`: The URL to scrape
- `CHROMEDRIVER_PORT`: The port to use for ChromeDriver
- `CHROMEDRIVER_BIN_DIR`: The directory to store ChromeDriver
- `OUTPUT_FILE`: The file to save results to
- `FORCE_DOWNLOAD`: Set to any value to force download a new ChromeDriver

## How It Works

This scraper:
1. Detects your installed Chrome version
2. Downloads a compatible ChromeDriver if needed
3. Starts ChromeDriver automatically
4. Uses Fantoccini to control a headless Chrome browser
5. Navigates to the NeetCode 150 page
6. Waits for the page to load and the JavaScript to render the content
7. Extracts problem titles and LeetCode links from the table
8. Outputs the results to the console and optionally saves to a file
9. Cleans up resources automatically

## Project Structure

- `src/main.rs`: Entry point for the application
- `src/lib.rs`: Library exports and documentation
- `src/config.rs`: Configuration management
- `src/error.rs`: Custom error types
- `src/models.rs`: Data structures
- `src/chromedriver.rs`: ChromeDriver management
- `src/scraper.rs`: Web scraping logic

## Output Format

The output is a list of problems in the format:
```
1. Problem Title - https://leetcode.com/problems/problem-slug/
```

If saving to a file, the output is a JSON array of objects with `title` and `leetcode_link` fields.

- The scraper requires an active internet connection
- The structure of the website may change over time, which could break the scraper
