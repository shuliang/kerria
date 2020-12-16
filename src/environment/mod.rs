mod jwt;

use clap::Clap;
use jwt::Jwt;
use sqlx::mysql::MySqlPool;
use std::net::SocketAddr;

#[derive(Clap, Debug)]
#[clap(
    name = "kerria-app",
    rename_all = "kebab-case",
    rename_all_env = "screaming-snake"
)]
pub struct Args {
    #[clap(short, long)]
    debug: bool,

    #[clap(required = true, short = 'D', long, env)]
    database_url: String,
    #[clap(required = true, short = 'R', long, env)]
    redis_url: String,

    #[clap(required = true, long, env)]
    jwt_secret: String,

    #[clap(default_value = "127.0.0.1:3000", env)]
    pub host: SocketAddr,
}

#[derive(Clone, Debug)]
pub struct Environment {
    db_pool: MySqlPool,
    jwt: Jwt,
}

impl Environment {
    pub async fn new(args: &Args) -> anyhow::Result<Self> {
        let Args {
            database_url,
            jwt_secret,
            ..
        } = &args;
        let db_pool = MySqlPool::connect(database_url).await?;
        let jwt = Jwt::new(&jwt_secret);
        Ok(Self { db_pool, jwt })
    }

    pub fn db(&self) -> &MySqlPool {
        &self.db_pool
    }

    pub fn jwt(&self) -> &Jwt {
        &self.jwt
    }
}
