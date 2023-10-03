use actix_web_utils::{ServiceResponse, x_u_res_db_or_sr};
use league_types::domain::sport::Sport;
use sqlx::PgConnection;

use crate::{dao::sport_dao, util::text_serializer};

pub async fn insert_all_sports_from_list(conn: &mut PgConnection) {
    // This adds a lot of time to the startup. Find a way to cancel it maybe?
    let persisted_sports = match sport_dao::get_all_sports_ordered(conn).await {
        Ok(sports) => sports,
        Err(error) => panic!("{:?}", error),
    };

    let all_sports = text_serializer::parse_sport_list();

    //println!("{:#?}", persisted_sports);
    if persisted_sports.len() == all_sports.len() {
        return;
    }

    for sport in all_sports {
        match sport_dao::insert_sport(conn, sport).await {
            Ok(_) => {}
            Err(error) => println!("{:?}", error),
        };
    }
}

pub async fn get_all_sports(conn: &mut PgConnection) -> ServiceResponse<Vec<Sport>> {
    Ok(x_u_res_db_or_sr!(sport_dao::get_all_sports_ordered(conn).await))
}