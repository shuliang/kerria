use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::models::admin::AdminUser;
use crate::models::admin::Claims;
use crate::models::AuthError;

const BEARER: &str = "Bearer ";

#[derive(Clone, Debug)]
pub struct Jwt {
    secret: String,
}

impl Jwt {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_owned(),
        }
    }

    pub fn encode(&self, claims: Claims) -> Result<String> {
        let header = Header::new(Algorithm::HS512);
        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?;
        Ok(token)
    }

    pub fn decode(&self, jwt_raw: Option<String>) -> Result<Claims> {
        match jwt_raw {
            None => return Err(AuthError::NoAuthHeaderError.into()),
            Some(v) => {
                if !v.starts_with(BEARER) {
                    return Err(AuthError::InvalidAuthHeaderError.into());
                }
                let jwt = v.trim_start_matches(BEARER);

                let decoded = decode::<Claims>(
                    &jwt,
                    &DecodingKey::from_secret(self.secret.as_bytes()),
                    &Validation::new(Algorithm::HS512),
                )
                .map_err(|_| AuthError::JWTTokenError)?;
                Ok(decoded.claims)
            }
        }
    }

    pub fn decode_to_admin_user(&self, jwt_raw: Option<String>) -> Result<AdminUser> {
        let claims = self.decode(jwt_raw)?;
        let id = claims.sub.parse::<u64>();
        if id.is_err() {
            return Err(AuthError::InvalidAuthHeaderError.into());
        }
        Ok(AdminUser {
            id: id.unwrap(),
            username: claims.name,
        })
    }
}
