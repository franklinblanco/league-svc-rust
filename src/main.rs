use dao::main_dao::{self, run_all_migrations};
use util::env_util;

#[forbid(unsafe_code)]

pub mod util;
pub mod dao;
pub mod routes;
pub mod domain;
pub mod dto;
pub mod service;
pub mod validation;

#[tokio::main]
async fn main() {
    //  Retrieve env variables and send to services that need them.
    let env_vars = env_util::get_dot_env_map();
    
    //  Start database
    let mut db_conn = match main_dao::start_database_connection(&env_vars).await {
        Ok(conn) => conn,
        Err(e) => panic!("Failure starting the database. Reason: {}", e)
    };

    //  Run all migrations
    run_all_migrations(&mut db_conn).await;

    //  Pass shared state to server and start it
    routes::main_router::start_all_routes(db_conn, env_vars).await.unwrap();
}