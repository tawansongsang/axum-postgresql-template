use lib_postgresql::model::ModelManager;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting Connection to Sql Server");

    // -- Initialize ModelManager.
    let mm = ModelManager::new().await.unwrap();

    // -- Testing Connection to DB
    mm.test_connection().await;

    info!("Starting Service ...");
}
