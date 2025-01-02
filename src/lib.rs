#[cfg(test)]
pub mod tests;

pub mod errors;

use std::{fs, path::Path};

use dirs;

pub fn run_sorting() -> Result<(), errors::custom_errors::CustomError> {
    tracing::info!("Starting the sorting process...");
    let download_dir = match dirs::download_dir() {
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

    let contents = match fs::read_dir(&download_dir) {
        Ok(iterator) => {
            tracing::info!("Download directory read successfully");
            iterator
        }
        Err(error) => {
            tracing::error!("Error reading the download directory: {:?}", error);
            return Err(errors::custom_errors::CustomError::Unknown);
        }
    };

    // I know this is an abomination, I don't care
    tracing::info!("Sorting the files...");
    for entry in contents {
        match entry {
            Ok(direntry) => {
                let path = direntry.path();
                if let Some(pathstr) = path.to_str() {
                    if let Some(ext) = Path::new(pathstr).extension().and_then(|s| s.to_str()) {
                        tracing::info!("File extension found: {}", ext);
                        let new_dir = download_dir.join(ext);
                        if !new_dir.exists() {
                            if let Err(error) = fs::create_dir(&new_dir) {
                                tracing::error!("Error creating directory: {:?}", error);
                                return Err(errors::custom_errors::CustomError::Unknown);
                            }
                            tracing::info!("Created directory: {:?}", new_dir);
                        }
                        let new_path = new_dir.join(direntry.file_name());
                        if let Err(error) = fs::rename(&path, &new_path) {
                            tracing::error!("Error moving file: {:?}", error);
                            return Err(errors::custom_errors::CustomError::Unknown);
                        }
                        tracing::info!("Moved file {:?} to {:?}", path, new_path);
                    } else {
                        tracing::warn!("No file extension found for file: {}", pathstr);
                    }
                } else {
                    tracing::error!("Error reading the path");
                    return Err(errors::custom_errors::CustomError::Unknown);
                }
            }
            Err(error) => {
                tracing::warn!("Error reading the download directory: {:?}", error);
            }
        }
    }
    Ok(())
}
