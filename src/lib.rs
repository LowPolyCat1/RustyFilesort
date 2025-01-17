use iced;
use tracing::instrument::WithSubscriber;

#[cfg(test)]
pub mod tests;

pub mod application;
pub mod auto_start;
pub mod errors;
pub mod sorting;
pub mod tray;

pub async fn run_programm() -> iced::Result {
    let file_appender = tracing_appender::rolling::minutely("./logs", "debug.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();

    tracing::info!("Hello, World! from RustyFilesort");

    iced::application(
        "RustyFilesort",
        application::iced_application::RustyFilesortApplication::update,
        application::iced_application::RustyFilesortApplication::view,
    )
    .theme(application::iced_application::RustyFilesortApplication::theme)
    .settings(application::settings::settings())
    .window(application::settings::window_settings())
    .with_current_subscriber()
    .into_inner()
    .run_with(application::iced_application::RustyFilesortApplication::new)
}
