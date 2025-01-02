#![windows_subsystem = "windows"]

use rusty_filesort::run_sorting;

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::minutely("./logs", "debug.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();

    tracing::info!("Hello, world!");
    match run_sorting() {
        Ok(_) => {
            tracing::info!("Sorting process completed successfully");
        }
        Err(error) => {
            tracing::error!("Error occurred: {:?}", error);
            panic!("Error occurred: {:?}", error);
        }
    }
}
