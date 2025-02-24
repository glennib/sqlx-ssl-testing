use clap::{Parser, ValueEnum};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    query, PgPool,
};
use std::{str::FromStr, time::Duration};

#[derive(Debug, Clone, Parser)]
struct Cli {
    #[arg(long, env = "DATABASE_URL")]
    database_url: String,
    #[arg(long, env = "PG_SSL_MODE")]
    pg_ssl_mode: PgSslModeOption,
    #[arg(long, default_value = "30", env = "DURATION_SECS")]
    duration_secs: u64,
    #[arg(long, default_value = "3", env = "QUERY_PERIOD_SECS")]
    query_period_secs: u64,
}

#[derive(Debug, Clone, ValueEnum)]
enum PgSslModeOption {
    Require,
    Disable,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let cli = Cli::parse();
    eprintln!("cli={cli:?}");
    let pg_ssl_mode = match cli.pg_ssl_mode {
        PgSslModeOption::Require => PgSslMode::Require,
        PgSslModeOption::Disable => PgSslMode::Disable,
    };
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(PgConnectOptions::from_str(&cli.database_url)?.ssl_mode(pg_ssl_mode))
        .await?;
    let period = Duration::from_secs(cli.query_period_secs);
    let _handle = tokio::spawn(async move {
        poll(pool, period).await;
    });

    tokio::time::sleep(Duration::from_secs(cli.duration_secs)).await;

    eprintln!("Done");

    Ok(())
}

async fn poll(pool: PgPool, period: Duration) {
    loop {
        let users = query!("select user_id, email from users order by user_id")
            .fetch_all(&pool)
            .await
            .expect("can fetch unless database connection has failed or schema is wrong");
        eprintln!("n_users={}", users.len());
        tokio::time::sleep(period).await;
    }
}
