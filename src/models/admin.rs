use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct AdminLoginResponse {
    pub username: String,
    pub token: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdminLoginUser {
    pub id: u64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdminUser {
    pub id: u64,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePassword {
    pub old_password: String,
    pub new_password: String,
}
