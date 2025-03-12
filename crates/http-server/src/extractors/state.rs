use database::sea_orm::{ConnectOptions, Database, DatabaseConnection};
use axum_macros::FromRef;
use crate::error::ServerResult;

#[derive(FromRef, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub async fn new(db_url: &str) -> ServerResult<AppState> {
        let mut opt = ConnectOptions::new(db_url);
        opt.sqlx_logging(false);

        let database_connection = Database::connect(opt).await?;

        database_connection.ping().await?;
        Ok(Self { db: database_connection })
    }
}