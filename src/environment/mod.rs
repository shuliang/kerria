use clap::Clap;
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
    #[clap(required = true, long, env)]
    argon_secret: String,
    #[clap(long, env)]
    argon_iterations: Option<u32>,
    #[clap(long, env)]
    argon_memory_size: Option<u32>,
    #[clap(short, long, env)]
    session_lifetime: Option<i64>,

    #[clap(default_value = "127.0.0.1:3000", env)]
    pub host: SocketAddr,
}

#[derive(Clone, Debug)]
pub struct Environment {
    db_pool: MySqlPool,
}

impl Environment {
    pub async fn new(args: &Args) -> anyhow::Result<Self> {
        let Args { database_url, .. } = &args;
        let db_pool = MySqlPool::connect(database_url).await?;
        Ok(Self { db_pool })
    }

    pub fn db(&self) -> &MySqlPool {
        &self.db_pool
    }
}
