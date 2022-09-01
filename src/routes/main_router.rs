use std::{sync::{Arc}, collections::HashMap};
use actix_web::{HttpServer, App, web};
use reqwest::Client;
use sqlx::MySqlPool;

use super::player::{create_player_profile, edit_player_profile};

///  This function is to be used in case code is meant to be run after server startup
pub async fn after_startup_fn() {
    //TODO: Gather all startup duties. 
    //TODO: Get all sports and store them in DB
    println!("{}", "Started server.");
}

pub async fn start_all_routes(db_conn: MySqlPool, env_vars: HashMap<String, String>)
-> Result<(), std::io::Error>
{
    //  Get env variables to build server address
    let host_addr: &str = match env_vars.get("HOST_ADDRESS") {
        Some(str) => {str},
        None => {panic!("HOST_ADDRESS env variable not found.");},
    };
    let host_port: u16 = match env_vars.get("HOST_PORT") {
        Some(str) => {
            match str.parse::<u16>() {
                Ok(resolved_port) => {resolved_port},
                Err(e) => {panic!("{:?}", e);}
            }
        },
        None => {panic!("HOST_PORT env variable not found.");},
    };

    //  Extract variables to be put into shared app state & clone them
    let db_conn_state = web::Data::new(Arc::new(db_conn));
    let env_vars_state = web::Data::new(Arc::new(env_vars.clone()));
    let client_state = web::Data::new(Arc::new(Client::new()));
    //  Start server code that turns into a future to be executed below
    let server_future = HttpServer::new( move || {
        App::new()
        //  Define routes & pass in shared state
            .app_data(db_conn_state.clone())
            .app_data(env_vars_state.clone())
            .app_data(client_state.clone())
            .service(web::scope("/")
                .service(create_player_profile)
                .service(edit_player_profile))
            //.service(user_routes::get_user_from_db)
    })
    .bind((host_addr, host_port))?
    .run();

    //  Actual server start and after startup call
    let (server_start_result, _after_startup_value) = 
    tokio::join!(server_future, after_startup_fn());
    return server_start_result; //   Return server
}