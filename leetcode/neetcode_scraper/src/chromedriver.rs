use crate::config::Config;
use crate::error::{Error, Result};
use crate::models::ChromeForTestingVersions;
use futures_util::StreamExt;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tempfile::TempDir;
use zip::ZipArchive;

/// Manages ChromeDriver download, installation, and execution
pub struct ChromeDriver {
    /// Path to the ChromeDriver executable
    pub path: PathBuf,

    /// The ChromeDriver process if running
    process: Option<Child>,

    /// Configuration
    config: Config,
}

impl ChromeDriver {
    /// Create a new ChromeDriver manager
    pub fn new(config: Config) -> Self {
        Self {
            path: config.bin_dir.join(Self::get_executable_name()),
            process: None,
            config,
        }
    }

    /// Get the executable name based on the platform
    pub fn get_executable_name() -> &'static str {
        if cfg!(target_os = "windows") {
            "chromedriver.exe"
        } else {
            "chromedriver"
        }
    }

    /// Check if ChromeDriver exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Set up ChromeDriver (find existing or download)
    pub async fn setup(&mut self) -> Result<()> {
        // Check if ChromeDriver already exists and we're not forcing a download
        if self.exists() && !self.config.force_download {
            log::info!("Found existing ChromeDriver at: {:?}", self.path);
            return Ok(());
        }

        // Get Chrome version
        let chrome_version = Self::check_chrome_version()?;
        log::info!("Detected Google Chrome version: {}", chrome_version);

        // Find compatible ChromeDriver
        let download_url = self.find_chromedriver_download(&chrome_version).await?;
        log::info!("Found compatible ChromeDriver at: {}", download_url);

        // Download and install ChromeDriver
        self.download_and_install(&download_url).await?;

        Ok(())
    }

    /// Start ChromeDriver
    pub fn start(&mut self) -> Result<()> {
        if self.process.is_some() {
            log::warn!("ChromeDriver is already running");
            return Ok(());
        }

        log::info!(
            "Starting ChromeDriver on port {}...",
            self.config.chromedriver_port
        );

        let process = Command::new(&self.path)
            .arg(format!("--port={}", self.config.chromedriver_port))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| Error::StartError(format!("Failed to start ChromeDriver: {}", e)))?;

        // Give ChromeDriver a moment to start
        std::thread::sleep(Duration::from_secs(1));

        self.process = Some(process);
        log::info!("ChromeDriver started successfully");

        Ok(())
    }

    /// Stop ChromeDriver
    pub fn stop(&mut self) -> Result<()> {
        if let Some(mut process) = self.process.take() {
            log::info!("Shutting down ChromeDriver...");
            match process.kill() {
                Ok(_) => {
                    log::info!("ChromeDriver terminated successfully");
                    Ok(())
                }
                Err(e) => {
                    let err = Error::StartError(format!("Failed to terminate ChromeDriver: {}", e));
                    log::error!("{}", err);
                    Err(err)
                }
            }
        } else {
            log::warn!("ChromeDriver is not running");
            Ok(())
        }
    }

    /// Check if Chrome is installed and get its version
    pub fn check_chrome_version() -> Result<String> {
        // Different commands for different platforms
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args([
                    "/C",
                    "reg",
                    "query",
                    "HKEY_CURRENT_USER\\Software\\Google\\Chrome\\BLBeacon",
                    "/v",
                    "version",
                ])
                .output()?
        } else if cfg!(target_os = "macos") {
            Command::new("sh")
                .args([
                    "-c",
                    "/Applications/Google\\ Chrome.app/Contents/MacOS/Google\\ Chrome --version",
                ])
                .output()?
        } else {
            // Linux
            Command::new("sh")
                .args(["-c", "google-chrome --version"])
                .output()?
        };

        if !output.status.success() {
            return Err(Error::ChromeNotFound(
                "Chrome not found or could not determine version".to_string(),
            ));
        }

        let version_output = String::from_utf8_lossy(&output.stdout);

        // Extract version number with regex-like parsing
        let version = version_output
            .split_whitespace()
            .find(|s| s.chars().any(|c| c.is_ascii_digit()))
            .unwrap_or("Unknown")
            .trim()
            .to_string();

        Ok(version)
    }

    /// Get the major version from a full version string (e.g., "114.0.5735.90" -> "114")
    fn get_major_version(version: &str) -> Option<u32> {
        version.split('.').next()?.parse::<u32>().ok()
    }

    /// Find the ChromeDriver download URL for the given Chrome version
    async fn find_chromedriver_download(&self, chrome_version: &str) -> Result<String> {
        let chrome_major = Self::get_major_version(chrome_version).ok_or_else(|| {
            Error::ChromeDriverVersionNotFound(format!(
                "Invalid Chrome version format: {}",
                chrome_version
            ))
        })?;

        // Fetch the Chrome for Testing versions JSON
        let url = "https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions-with-downloads.json";
        let response = reqwest::get(url).await?;
        let versions_data: ChromeForTestingVersions = response.json().await?;

        // Find a matching version
        let platform = if cfg!(target_os = "windows") {
            "win64"
        } else if cfg!(target_os = "macos") {
            if cfg!(target_arch = "aarch64") {
                "mac-arm64"
            } else {
                "mac-x64"
            }
        } else {
            "linux64"
        };

        // Check if the Stable channel version matches our Chrome version
        let stable_version = &versions_data.channels.stable.version;
        let stable_major = Self::get_major_version(stable_version).unwrap_or(0);

        if stable_major == chrome_major {
            // Look for the ChromeDriver download for our platform
            for download in &versions_data.channels.stable.downloads.chromedriver {
                if download.platform == platform {
                    return Ok(download.url.clone());
                }
            }
        }

        // If no match in Stable, try Beta
        let beta_version = &versions_data.channels.beta.version;
        let beta_major = Self::get_major_version(beta_version).unwrap_or(0);

        if beta_major == chrome_major {
            for download in &versions_data.channels.beta.downloads.chromedriver {
                if download.platform == platform {
                    return Ok(download.url.clone());
                }
            }
        }

        // If still no match, try Dev and Canary channels
        let dev_version = &versions_data.channels.dev.version;
        let dev_major = Self::get_major_version(dev_version).unwrap_or(0);

        if dev_major == chrome_major {
            for download in &versions_data.channels.dev.downloads.chromedriver {
                if download.platform == platform {
                    return Ok(download.url.clone());
                }
            }
        }

        let canary_version = &versions_data.channels.canary.version;
        let canary_major = Self::get_major_version(canary_version).unwrap_or(0);

        if canary_major == chrome_major {
            for download in &versions_data.channels.canary.downloads.chromedriver {
                if download.platform == platform {
                    return Ok(download.url.clone());
                }
            }
        }

        Err(Error::ChromeDriverVersionNotFound(format!(
            "No compatible ChromeDriver found for Chrome version {}",
            chrome_version
        )))
    }

    /// Download and install ChromeDriver
    async fn download_and_install(&self, download_url: &str) -> Result<()> {
        // Create bin directory if it doesn't exist
        fs::create_dir_all(&self.config.bin_dir)?;

        // Create a temporary directory for downloads
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        // Download the ChromeDriver zip file
        let zip_path = temp_path.join("chromedriver.zip");
        self.download_file(download_url, &zip_path).await?;

        // Create a temporary directory for extraction
        let extract_dir = temp_path.join("extracted");
        fs::create_dir_all(&extract_dir)?;

        // Extract the zip file
        self.extract_zip(&zip_path, &extract_dir)?;

        // Find the chromedriver executable in the extracted files
        let mut source_path = None;
        let executable_name = Self::get_executable_name();

        for entry in fs::read_dir(&extract_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Look inside subdirectories
                for subentry in fs::read_dir(&path)? {
                    let subentry = subentry?;
                    let subpath = subentry.path();

                    if subpath.file_name().and_then(|n| n.to_str()) == Some(executable_name) {
                        source_path = Some(subpath);
                        break;
                    }
                }
            } else if path.file_name().and_then(|n| n.to_str()) == Some(executable_name) {
                source_path = Some(path);
                break;
            }
        }

        // Copy the executable to the bin directory
        match source_path {
            Some(path) => {
                log::info!("Found ChromeDriver at: {:?}", path);
                log::info!("Copying to bin directory: {:?}", self.path);

                fs::copy(&path, &self.path)?;

                // Set executable permissions on Unix-like systems
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = fs::metadata(&self.path)?.permissions();
                    perms.set_mode(0o755); // rwxr-xr-x permissions
                    fs::set_permissions(&self.path, perms)?;
                }

                log::info!("ChromeDriver installed successfully");
                Ok(())
            }
            None => Err(Error::DownloadError(
                "ChromeDriver executable not found in the extracted files".to_string(),
            )),
        }
    }

    /// Download a file from a URL to a specified path
    async fn download_file(&self, url: &str, output_path: &Path) -> Result<()> {
        log::info!("Downloading from {}...", url);

        // Create a reqwest client and get the file
        let response = reqwest::get(url).await?;

        // Check if the request was successful
        if !response.status().is_success() {
            return Err(Error::DownloadError(format!(
                "Failed to download: HTTP status {}",
                response.status()
            )));
        }

        // Create the output file
        let mut file = File::create(output_path)?;

        // Stream the download to the file
        let mut stream = response.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            file.write_all(&chunk)?;
        }

        log::info!("Download completed to {:?}", output_path);
        Ok(())
    }

    /// Extract a zip file to a specified directory
    fn extract_zip(&self, zip_path: &Path, output_dir: &Path) -> Result<()> {
        log::info!("Extracting {:?} to {:?}", zip_path, output_dir);

        // Open the zip file
        let file = File::open(zip_path)?;
        let mut archive = ZipArchive::new(file)?;

        // Extract each file in the archive
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = output_dir.join(file.name());

            if file.name().ends_with('/') {
                // Create directory if it doesn't exist
                fs::create_dir_all(&outpath)?;
            } else {
                // Create parent directory if it doesn't exist
                if let Some(parent) = outpath.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)?;
                    }
                }

                // Extract file
                let mut outfile = File::create(&outpath)?;
                io::copy(&mut file, &mut outfile)?;

                // Set executable permissions on Unix-like systems
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if outpath.to_string_lossy().contains("chromedriver") {
                        let mut perms = fs::metadata(&outpath)?.permissions();
                        perms.set_mode(0o755); // rwxr-xr-x permissions
                        fs::set_permissions(&outpath, perms)?;
                    }
                }
            }
        }

        log::info!("Extraction completed");
        Ok(())
    }
}

impl Drop for ChromeDriver {
    fn drop(&mut self) {
        if self.process.is_some() {
            let _ = self.stop();
        }
    }
}
