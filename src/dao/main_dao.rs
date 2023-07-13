use std::collections::HashMap;

use sqlx::PgPool;

pub async fn start_database_connection(
    env_vars: &HashMap<String, String>,
) -> Result<PgPool, sqlx::Error> {
    let db_url = match env_vars.get("DATABASE_URL") {
        Some(str) => str,
        None => panic!("DATABASE_URL env var not found"),
    };
    let formatted_db_url = &db_url;
    sqlx::PgPool::connect(&formatted_db_url).await
}
pub async fn run_all_migrations(conn: &PgPool) {
    match sqlx::migrate!("./migrations").run(conn).await {
        Ok(()) => {
            println!("{}", "Successfully ran migrations.")
        }
        Err(error) => {
            panic!("{error}")
        }
    }
}
