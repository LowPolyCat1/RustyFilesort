#[cfg(test)]
pub mod tests;

use tokio::fs::{self, ReadDir};

pub async fn run_sorting() {
    let download_dir = fs::read_dir("Downloads").await;
    let inside_download_dir: ReadDir = match download_dir {
        Ok(content) => content,
        Err(error) => {
            tracing::error!("Error reading directory: {:?}", error);
            return;
        }
    };
    println!("{:?}", inside_download_dir)
}
