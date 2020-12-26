use anyhow::{anyhow, Result};
use argon2::Config;
use rand::Rng;
use serde_json::json;
use warp::http::StatusCode;

use crate::environment::Environment;
use crate::models::admin::{
    AdminLoginRequest, AdminLoginResponse, AdminUser, Claims, UpdatePassword,
};
use crate::models::AuthError;
use crate::sql;

pub async fn login_handler(env: Environment, req: AdminLoginRequest) -> Result<impl warp::Reply> {
    let res = sql::admin::get_user(env.db(), &req.username).await?;
    match res {
        Some(user) => {
            let is_valid = verify_password(&user.password, req.password.as_bytes())?;
            if is_valid {
                let expiry = chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::days(1))
                    .expect("valid timestamp")
                    .timestamp();
                let claims = Claims {
                    sub: user.id.to_string(),
                    name: user.username,
                    exp: expiry as usize,
                };
                let token = env.jwt().encode(claims)?;
                let reply = warp::reply::json(&AdminLoginResponse {
                    username: req.username,
                    token,
                    avatar: None,
                });
                return Ok(reply);
            } else {
                return Err(AuthError::InvalidCredentials.into());
            }
        }
        None => return Err(AuthError::InvalidUserName.into()),
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

pub async fn get_current_user_handler(
    env: Environment,
    user: AdminUser,
    jwt: String,
) -> Result<impl warp::Reply> {
    let token = env.jwt().trim_token(jwt)?;
    let reply = warp::reply::json(&AdminLoginResponse {
        username: user.username,
        token,
        avatar: None,
    });
    return Ok(reply);
}

pub async fn update_password_handler(
    env: Environment,
    user: AdminUser,
    jwt: String,
    req: UpdatePassword,
) -> Result<impl warp::Reply> {
    if req.new_password.len() < 12 {
        return Err(anyhow!("password length must not be shorter than 12."));
    }
    let res = sql::admin::get_user(env.db(), &user.username).await?;
    match res {
        None => return Err(AuthError::InvalidUserName.into()),
        Some(u) => {
            let is_valid = verify_password(&u.password, req.old_password.as_bytes())?;
            if !is_valid {
                return Err(AuthError::InvalidCredentials.into());
            }
            let new_pw = hash_password(req.new_password.as_bytes())?;
            sql::admin::update_password(env.db(), &user.username, &new_pw).await?;

            let token = env.jwt().trim_token(jwt)?;
            let reply = warp::reply::json(&AdminLoginResponse {
                username: user.username,
                token,
                avatar: None,
            });
            Ok(reply)
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

fn verify_password(password1: &str, password2: &[u8]) -> Result<bool> {
    argon2::verify_encoded(password1, password2).map_err(|_| AuthError::InvalidCredentials.into())
}
