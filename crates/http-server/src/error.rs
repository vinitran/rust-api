use std::borrow::Cow;
use axum_derive_error::ErrorResponse;
use axum::http::StatusCode;


#[allow(dead_code)]
#[derive(ErrorResponse, thiserror::Error)]
pub enum ServerError {
    #[error("{0}")]
    EnvError(Cow<'static, str>),

    #[error("{0:#?}")]
    Internal(Cow<'static, str>),

    #[error(transparent)]
    Database(#[from] database::sea_orm::error::DbErr),

    #[error("{0:#?}")]
    Custom(Cow<'static, str>),

    #[error("{0:#?}")]
    #[status(StatusCode::UNAUTHORIZED)]
    Unauthorized(Cow<'static, str>),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

pub type ServerResult<A> = Result<A, ServerError>;

impl From<shared::env::EnvError> for ServerError {
    fn from(value: shared::env::EnvError) -> Self {
        Self::EnvError(value.0.into())
    }
}
