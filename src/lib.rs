#[cfg(test)]
pub mod tests;

pub mod errors;

use std::{
    self,
    path::{self, PathBuf},
};

use dirs;
use tokio;

pub async fn run_sorting() -> Result<(), errors::custom_errors::CustomError> {
    tracing::info!("Starting the sorting process...");
    tracing::info!("Finding the download directory...");
    let download_dir = dirs::download_dir();
    let download_dir: std::path::PathBuf = match download_dir {
        Some(pathbuf) => {
            tracing::info!("Download directory found: {:?}", pathbuf);
            pathbuf
        }
        None => {
            tracing::error!("Could not find the download directory");
            return Err(errors::custom_errors::CustomError::DirectoryNotFound(
                "Download directory not found".to_string(),
            ));
        }
    };
    tracing::info!("Reading the download directory...");
    let contents = std::fs::read_dir(download_dir.clone());
    let content = match contents {
        Ok(iterator) => {
            tracing::info!("Download directory read successfully");
            iterator
        }
        Err(error) => {
            tracing::error!("Error reading the download directory: {:?}", error);
            return Err(errors::custom_errors::CustomError::Unknown);
        }
    };
    tracing::info!("Sorting the files...");
    for entry in content {
        match entry {
            Ok(direntry) => {
                let path = direntry.path();
                let path = path.to_str();
                match path {
                    Some(pathstr) => {
                        let extension = std::path::Path::new(pathstr)
                            .extension()
                            .and_then(std::ffi::OsStr::to_str);

                        match extension {
                            Some(ext) => {
                                tracing::info!("File extension found: {}", ext);
                                let new_dir = download_dir.join(ext);
                                if !new_dir.exists() {
                                    match std::fs::create_dir(&new_dir) {
                                        Ok(_) => {}
                                        Err(error) => {
                                            tracing::error!(
                                                "Error creating directory: {:?}",
                                                error
                                            );
                                            return Err(
                                                errors::custom_errors::CustomError::Unknown,
                                            );
                                        }
                                    }
                                    tracing::info!("Created directory: {:?}", new_dir);
                                }
                                let new_path = new_dir.join(direntry.file_name());
                                let path = std::path::Path::new(pathstr);
                                match std::fs::rename(&path, &new_path) {
                                    Ok(_) => {}
                                    Err(error) => {
                                        tracing::error!("Error moving file: {:?}", error);
                                        return Err(errors::custom_errors::CustomError::Unknown);
                                    }
                                }
                                tracing::info!("Moved file {:?} to {:?}", path, new_path);
                            }
                            None => {
                                tracing::warn!("No file extension found for file: {}", pathstr);
                                tracing::warn!("Skipping the current entry");
                                continue;
                            }
                        }
                    }
                    None => {
                        tracing::error!("Error reading the path");
                        return Err(errors::custom_errors::CustomError::Unknown);
                    }
                }
            }
            Err(error) => {
                tracing::warn!("Error reading the download directory: {:?}", error);
                tracing::warn!("Skipping the current entry");
                continue;
            }
        };
    }
    Result::Ok(())
}
