use std::{sync::{Arc}, collections::HashMap};
use actix_web::{HttpServer, App, web};
use chrono::Utc;
use reqwest::Client;
use sqlx::MySqlPool;

use crate::service::sport::insert_all_sports_from_list;

use super::{player, sport, league, place, league_player};


///  This function is to be used in case code is meant to be run after server startup
pub async fn after_startup_fn(conn: &MySqlPool, start_time: i64) {
    insert_all_sports_from_list(conn).await;
    println!("{}", "Finished db updates!");
    println!("{}", "Started server with no errors!");
    println!("Server took {}ms to start", Utc::now().timestamp_millis() - start_time);
}

pub async fn start_all_routes(db_conn: MySqlPool, env_vars: HashMap<String, String>, start_time: i64)
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
    let db_conn_state = web::Data::new(Arc::new(db_conn.clone()));
    let env_vars_state = web::Data::new(Arc::new(env_vars.clone()));
    let client_state = web::Data::new(Arc::new(Client::new()));
    //  Start server code that turns into a future to be executed below
    let server_future = HttpServer::new( move || {
        App::new()
        //  Define routes & pass in shared state
            .app_data(db_conn_state.clone())
            .app_data(env_vars_state.clone())
            .app_data(client_state.clone())
            .service(web::scope("/player")
                .service(player::create_player_profile)
                .service(player::edit_player_profile))

            .service(web::scope("/league")
                .service(league::create_league)
                .service(league::get_open_leagues_in_my_area)
                .service(league::get_specific_league))

            .service(web::scope("/sport")
                .service(sport::get_all_sports))

            .service(web::scope("/place")
                .service(place::get_places_for_country)
                .service(place::get_places_for_sport)
                .service(place::get_places_near_me))

            .service(web::scope("/league_player")
                .service(league_player::get_all_leagues_player_has_applied_to)
                .service(league_player::get_all_players_in_league)
                .service(league_player::get_league_request_status)
                .service(league_player::request_to_join_league))
            //.service(user_routes::get_user_from_db)
    })
    .bind((host_addr, host_port))?
    .run();

    //  Actual server start and after startup call
    let (server_start_result, _after_startup_value) = 
    tokio::join!(server_future, after_startup_fn(&db_conn, start_time));
    return server_start_result; //   Return server
}