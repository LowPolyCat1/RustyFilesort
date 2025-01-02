use auto_sort::run_sorting;

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::minutely("./logs", "debug.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();
    tracing::info!("Hello, world!");
    run_sorting().await;
}
