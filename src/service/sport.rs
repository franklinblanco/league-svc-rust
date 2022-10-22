use sqlx::MySqlPool;

use crate::{dao::sport_dao, util::text_serializer};

pub async fn insert_all_sports_from_list(conn: &MySqlPool) {
    // This adds a lot of time to the startup. Find a way to cancel it maybe?
    let persisted_sports = match sport_dao::get_all_sports_ordered(conn).await {
        Ok(sports) => sports,
        Err(err) => panic!("{:?}", err.error),
    };

    let all_sports = text_serializer::parse_sport_list();

    //println!("{:#?}", persisted_sports);
    if persisted_sports.len() == all_sports.len() {
        return;
    }

    let mut tx = conn.begin().await.unwrap();
    for sport in all_sports {
        match sport_dao::insert_sport(&mut tx, sport).await {
            Ok(_) => {}
            Err(err) => println!("{:?}", err.error),
        };
    }
    tx.commit().await.unwrap();
}
