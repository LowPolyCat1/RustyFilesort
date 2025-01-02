#[cfg(test)]
pub mod tests;

pub mod errors;

use dirs;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tokio;

pub fn get_download_dir() -> Result<PathBuf, errors::custom_errors::CustomError> {
    match dirs::download_dir() {
        Some(pathbuf) => {
            tracing::info!("Download directory found: {:?}", pathbuf);
            Ok(pathbuf)
        }
        None => {
            tracing::error!("Could not find the download directory");
            return Err(errors::custom_errors::CustomError::DirectoryNotFound(
                "Download directory not found".to_string(),
            ));
        }
    }
}

pub fn get_content(download_dir: &Path) -> Result<fs::ReadDir, errors::custom_errors::CustomError> {
    match fs::read_dir(download_dir) {
        Ok(iterator) => {
            tracing::info!("Download directory read successfully");
            Ok(iterator)
        }
        Err(error) => {
            tracing::error!("Error reading the download directory: {:?}", error);
            return Err(errors::custom_errors::CustomError::Unknown);
        }
    }
}

pub fn run_sorting() -> Result<(), errors::custom_errors::CustomError> {
    let download_dir = get_download_dir()?;

    let contents = get_content(&download_dir)?;

    // I know this is an abomination, I don't care
    tracing::info!("Sorting the files...");
    let mut futures = Vec::new();
    for entry in contents {
        let direntry = match entry {
            Ok(direntry) => direntry,
            Err(error) => {
                tracing::error!("Error reading the download directory: {:?}", error);
                continue;
            }
        };

        let path = direntry.path();
        let pathstr = match path.to_str() {
            Some(pathstr) => pathstr,
            None => {
                tracing::error!("Error reading the path");
                return Err(errors::custom_errors::CustomError::Unknown);
            }
        };

        let extension = match Path::new(pathstr).extension().and_then(|s| s.to_str()) {
            Some(extension) => extension,
            None => {
                tracing::warn!("No file extension found for file: {}", pathstr);
                continue;
            }
        };

        tracing::info!("File extension found: {}", extension);
        let new_dir = download_dir.join(extension);
        if !new_dir.exists() {
            if let Err(error) = fs::create_dir(&new_dir) {
                tracing::error!("Error creating directory: {:?}", error);
                // return Err(errors::custom_errors::CustomError::Unknown);
                continue;
            }
            tracing::info!("Created directory: {:?}", new_dir);
        }

        let new_path = new_dir.join(direntry.file_name());
        // if let Err(error) = tokio::fs::rename(&path, &new_path) {
        //     tracing::error!("Error moving file: {:?}", error);
        //     // return Err(errors::custom_errors::CustomError::Unknown);
        //     continue;
        // }
        futures.push(tokio::spawn(async move {
            if let Err(error) = tokio::fs::rename(&path, &new_path).await {
                tracing::error!("Error moving file: {:?}", error);
            } else {
                tracing::info!("Moved file {:?} to {:?}", path, new_path);
            }
        }));

        // tracing::info!("Moved file {:?} to {:?}", path, new_path);
    }
    Ok(())
}
