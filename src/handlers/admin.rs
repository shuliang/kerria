use anyhow::{anyhow, Result};
use argon2::Config;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use serde_json::json;
use warp::http::StatusCode;

use crate::environment::Environment;
use crate::models::admin::{
    AdminLoginRequest, AdminLoginResponse, AdminUser, Claims, UpdatePassword,
};
use crate::models::AuthError;
use crate::sql;

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"sy#S7;g1@m&Y";

pub async fn login_handler(env: Environment, req: AdminLoginRequest) -> Result<impl warp::Reply> {
    let res = sql::admin::get_user(env.db(), &req.username).await?;
    match res {
        Some(user) => {
            let is_valid = verify_password(user.password, req.password)?;
            if is_valid {
                let token = create_jwt(user.id, user.username)?;
                let reply = warp::reply::json(&AdminLoginResponse { token });
                return Ok(reply);
            } else {
                return Err(AuthError::InvalidCredentials.into());
            }
        }
        None => return Err(AuthError::InvalidUserName.into()),
    }
}

fn create_jwt(id: u64, username: String) -> Result<String> {
    let expiry = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(2))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: id.to_string(),
        name: username,
        exp: expiry as usize,
    };
    let header = Header::new(Algorithm::HS512);
    let token = encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))?;
    Ok(token)
}

pub async fn auth_handler(jwt_raw: Option<String>) -> Result<AdminUser> {
    match jwt_raw {
        None => return Err(AuthError::NoAuthHeaderError.into()),
        Some(v) => {
            if !v.starts_with(BEARER) {
                return Err(AuthError::InvalidAuthHeaderError.into());
            }
            let jwt = v.trim_start_matches(BEARER);
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| AuthError::JWTTokenError)?;
            let id = decoded.claims.sub.parse::<u64>();
            if id.is_err() {
                return Err(AuthError::InvalidAuthHeaderError.into());
            }
            return Ok(AdminUser {
                id: id.unwrap(),
                username: decoded.claims.name,
            });
        }
    }
}

pub async fn create_user_handler(
    env: Environment,
    user: AdminUser,
    req: AdminLoginRequest,
) -> Result<impl warp::Reply> {
    if user.username != "admin".to_string() {
        return Err(AuthError::NoPermissionError.into());
    }
    let pw = hash_password(req.password.as_bytes())?;
    sql::admin::create_user(env.db(), &req.username, &pw, user.username.as_str()).await?;
    let reply = warp::reply::json(&json!({"username": req.username, "password": req.password}));
    let reply = warp::reply::with_status(reply, StatusCode::CREATED);
    Ok(reply)
}

pub async fn update_password_handler(
    env: Environment,
    user: AdminUser,
    req: UpdatePassword,
) -> Result<impl warp::Reply> {
    if req.new_password.len() < 12 {
        return Err(anyhow!("password length must not be shorter than 12."));
    }
    let res = sql::admin::get_user(env.db(), &user.username).await?;
    match res {
        None => return Err(AuthError::InvalidUserName.into()),
        Some(u) => {
            let is_valid = verify_password(u.password, req.old_password)?;
            if !is_valid {
                return Err(AuthError::InvalidCredentials.into());
            }
            let new_pw = hash_password(req.new_password.as_bytes())?;
            sql::admin::update_password(env.db(), user.username, new_pw).await?;
            Ok(StatusCode::OK)
        }
    }
}

// encrypt

fn hash_password(password: &[u8]) -> Result<String> {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();
    let encode =
        argon2::hash_encoded(password, &salt, &config).map_err(|_| AuthError::EncryptError)?;
    Ok(encode)
}

fn verify_password(password1: String, password2: String) -> Result<bool> {
    argon2::verify_encoded(password1.as_str(), password2.as_bytes())
        .map_err(|_| AuthError::InvalidCredentials.into())
}
