#![windows_subsystem = "windows"]

use iced;
use rusty_filesort::run_programm;

#[tokio::main]
async fn main() -> iced::Result {
    run_programm().await
}
