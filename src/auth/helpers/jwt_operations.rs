use async_graphql::{Error, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use entities::user::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: i32,
    pub version: i16,
}

impl From<Model> for AuthUser {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            version: model.version,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
    user: AuthUser,
}

impl Claims {
    fn create_token(
        token_type: &TokenType,
        user: &Model,
        secret: &str,
        exp: i64,
        iss: String,
        sub: String,
    ) -> Result<String> {
        let now = Utc::now();
        let claims = Claims {
            sub,
            iss,
            iat: now.timestamp(),
            exp: (now + Duration::seconds(exp)).timestamp(),
            user: AuthUser::from(user.clone()),
        };

        if let Ok(token) = encode(
            &match token_type {
                TokenType::Access => Header::new(Algorithm::RS256),
                _ => Header::default(),
            },
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        ) {
            Ok(token)
        } else {
            Err(Error::new("Could not create token"))
        }
    }

    fn decode_token(secret: &str, token: &str) -> Result<AuthUser> {
        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        );

        match claims {
            Ok(s) => Ok(s.claims.user),
            _ => Err(Error::from("Invalid token")),
        }
    }
}

pub enum TokenType {
    Access,
    Reset,
    Confirmation,
    Refresh,
}

pub fn create_token(
    token_type: TokenType,
    user: &Model,
    secret: &str,
    exp: i64,
    iss: &str,
) -> Result<String> {
    let sub = match token_type {
        TokenType::Access => "access".to_owned(),
        TokenType::Reset => "reset".to_owned(),
        TokenType::Confirmation => "confirmation".to_owned(),
        TokenType::Refresh => "refresh".to_owned(),
    };

    Claims::create_token(&token_type, user, secret, exp, iss.to_owned(), sub)
}

pub fn decode_token(token: &str, secret: &str) -> Result<AuthUser> {
    Claims::decode_token(secret, token)
}

pub fn decode_access_token(token: &str, public_key: &str) -> Result<AuthUser> {
    let key = DecodingKey::from_secret(public_key.as_bytes());
    let validation = Validation::new(Algorithm::RS256);

    if let Ok(decoded) = decode::<Claims>(token, &key, &validation) {
        Ok(decoded.claims.user)
    } else {
        Err(Error::from("Could not decode access token"))
    }
}
