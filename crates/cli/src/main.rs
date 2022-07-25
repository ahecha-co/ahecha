use ahecha_cli::config::sync_config_from_db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv::dotenv()?;
  sync_config_from_db().await
}
