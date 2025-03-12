use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use serde::Serialize;
use crate::error::{ServerError, ServerResult};
use crate::error::ServerError::Unauthorized;
use crate::extractors::security::Claims;

pub mod login;

#[derive(Serialize)]
pub struct Tokens {
    access_token: String,
    renew_token: String,
}

pub fn generate_token(user_id: i64) -> ServerResult<Tokens> {
    let secret = shared::env::read_env("JWT_SECRET")?;
    let renew_secret = shared::env::read_env("JWT_RENEW_SECRET")?;

    let exp = Utc::now()
        .checked_add_signed(Duration::hours(6))
        .ok_or(ServerError::Custom(
            "get none from checked_add_signed()".into(),
        ))?
        .timestamp() as u32;

    let renew_exp = Utc::now()
        .checked_add_signed(Duration::weeks(3))
        .ok_or(ServerError::Custom(
            "get none from checked_add_signed()".into(),
        ))?
        .timestamp() as u32;

    let claims = Claims {
        id: user_id,
        exp: exp,
    };

    let renew_claims = Claims {
        id: user_id,
        exp: renew_exp,
    };

    let header = Header::new(Algorithm::HS256);

    let secret_key = EncodingKey::from_secret(secret.as_bytes());
    let renew_secret_key = EncodingKey::from_secret(renew_secret.as_bytes());

    let access_token = jsonwebtoken::encode(&header, &claims, &secret_key)
        .map_err(|e| ServerError::Custom(e.to_string().into()))?;

    let renew_token = jsonwebtoken::encode(&header, &renew_claims, &renew_secret_key)
        .map_err(|e| ServerError::Custom(e.to_string().into()))?;

    Ok(Tokens {
        access_token,
        renew_token,
    })
}

pub fn decode_token(token: &str, secret: String) -> ServerResult<Claims> {
    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
        .map_err(|err| match err.kind() {
            ErrorKind::ExpiredSignature => Unauthorized("Expired token".into()),
            _ => Unauthorized("Invalid token".into()),
        })
        .map(|token_data| token_data.claims)
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Duration, Utc};
    use jsonwebtoken::{Algorithm, EncodingKey, Header};
    use crate::extractors::security::Claims;

    #[test]
    fn generate_token_test() {
        let secret = "KXhYdlDQzAlB4UnQZfFhe08EuP34q6pOneYY39k9Zv8".to_string();
        let renew_secret = "KXhYdlDQzAlB4UnQZfFhe08EuP34q6pOneYY39k9Zv8".to_string();

        let datetime_str = "2024-03-12T14:30:00Z".to_string();
        let time : DateTime<Utc> = datetime_str.parse().expect("Invalid datetime format");

        let exp = match time
            .checked_add_signed(Duration::hours(6)) {
            Some(t) => t.timestamp() as u32,
            None => {
                eprintln!("Error: get none from checked_add_signed()");
                return;
            }
        };

        let renew_exp = match time
            .checked_add_signed(Duration::weeks(3)) {
            Some(t) => t.timestamp() as u32,
            None => {
                eprintln!("Error: get none from checked_add_signed()");
                return;
            }
        };

        let claims = Claims {
            id: 1,
            exp: exp,
        };

        let renew_claims = Claims {
            id: 1,
            exp: renew_exp,
        };

        let header = Header::new(Algorithm::HS256);

        let secret_key = EncodingKey::from_secret(secret.as_bytes());
        let renew_secret_key = EncodingKey::from_secret(renew_secret.as_bytes());

        let access_token = jsonwebtoken::encode(&header, &claims, &secret_key);
        assert!(access_token.is_ok(), "Failed to generate access token");

        let renew_token = jsonwebtoken::encode(&header, &renew_claims, &renew_secret_key);
        assert!(renew_token.is_ok(), "Failed to generate refresh token");

        assert_eq!(access_token.unwrap(), "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3MTAyNzU0MDAsImlkIjoxfQ.UNuozaiVkW1t8P3QvpcA_FVg9x6rTfQqXV1pWYRg-Qs");
        assert_eq!(renew_token.unwrap(), "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3MTIwNjgyMDAsImlkIjoxfQ.jfaonHfd2fN6HzeeqPLdM37RjFynl2G9uhL8ZiyA524");
    }
}
