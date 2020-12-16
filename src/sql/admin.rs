use anyhow::Result;
use sqlx::mysql::MySqlPool;
use sqlx::{query_as_unchecked, query_unchecked, Done};

use crate::models::admin::AdminLoginUser;

pub async fn get_user(db: &MySqlPool, username: &str) -> Result<Option<AdminLoginUser>> {
    query_as_unchecked!(
        AdminLoginUser,
        r#"
SELECT `id`, `username`, `password`
FROM admin_user
WHERE username = ?
"#,
        username
    )
    .fetch_optional(db)
    .await
    .map_err(|e| e.into())
}

pub async fn create_user(
    db: &MySqlPool,
    username: &str,
    password: &str,
    operator: &str,
) -> Result<u64> {
    let id = query_unchecked!(
        r#"
INSERT INTO admin_user (`username`, `password`, `creator`)
VALUES (?, ?, ?)
"#,
        username,
        password,
        operator
    )
    .execute(db)
    .await?
    .last_insert_id();
    Ok(id)
}

pub async fn update_password(db: &MySqlPool, username: String, password: String) -> Result<bool> {
    let row = query_unchecked!(
        r#"
UPDATE admin_user SET `password` = ?
WHERE `username` = ?  
"#,
        password,
        username
    )
    .execute(db)
    .await?
    .rows_affected();

    Ok(row > 0)
}
