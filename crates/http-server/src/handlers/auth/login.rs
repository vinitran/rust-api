use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use database::repositories::user;
use database::sea_orm::{DatabaseConnection};
use crate::error::ServerResult;

#[derive(Deserialize)]
pub struct Payload {
    pub username: Option<String>,
    pub first_name: Option<String>,
}

pub async fn handler(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i64>,
    Json(payload): Json<Payload>,
) -> ServerResult<Json<i64>> {
    let user_id = user::create_user_if_not_exist(&db, user_id, payload.username, payload.first_name).await?;

    Ok(Json(user_id))
}