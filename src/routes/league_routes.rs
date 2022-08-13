use actix_web::{HttpResponse, post, get, web};


#[post("/league")]
pub async fn create_league() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/league")]
pub async fn get_recent_leagues_near_me() -> HttpResponse {
    HttpResponse::Ok().finish()
}
#[get("/league/{league_id}")]
pub async fn get_specific_league(_league_id: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
#[get("/league/player/{player_id}")]
pub async fn get_leagues_hosted_by_player(_player_id: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
#[get("/league/place/{place_id}")]
pub async fn get_leagues_in_place(_place_id: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().finish()
}