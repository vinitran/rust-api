use serde::Serialize;

pub mod login;

#[derive(Serialize)]
pub struct Tokens {
    access_token: String,
    refresh_token: String,
}

// pub fn generate_token(user_id: String) -> ServerResult<Tokens> {
//     let secret = shared::env::read_env("JWT_SECRET")?;
//     let renew_secret = shared::env::read_env("JWT_RENEW_SECRET")?;
//
//     let exp = Utc::now()
//         .checked_add_signed(Duration::hours(6))
//         .ok_or(ServerError::Custom(
//             "get none from checked_add_signed()".into(),
//         ))?
//         .timestamp() as u32;
//
//     let renew_exp = Utc::now()
//         .checked_add_signed(Duration::weeks(3))
//         .ok_or(ServerError::Custom(
//             "get none from checked_add_signed()".into(),
//         ))?
//         .timestamp() as u32;
//
// }