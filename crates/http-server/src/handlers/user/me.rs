use serde::Serialize;
use axum::{extract::State, Json};
use axum::extract::Path;
use database::repositories;
use database::sea_orm::DatabaseConnection;
use crate::error::{ServerError, ServerResult};

#[derive(Serialize)]
pub struct Me {
    id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
}

pub async fn handler(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i64>,
) -> ServerResult<Json<Me>> {
    let user = repositories::user::find_by_id(&db, user_id).await?.ok_or(ServerError::Internal(
        format!("not found user with id {}", user_id).into(),
    ))?;

    Ok(Json(Me {
        id: user.id,
        username: user.username,
        first_name: user.first_name,
    }))
}

