use actix_web_utils::{ServiceResponse, x_u_res_db_or_sr, service_error};
use err::ServiceError as SE;
use league_types::{domain::trust::Trust, dto::trust::TrustRequestDto};
use sqlx::PgConnection;

use crate::dao::{player_dao::get_player_with_id, trust_dao};

pub async fn add_trusted_player(
    transaction: &mut PgConnection,
    trust_req: TrustRequestDto,
    user_id: i32,
) -> ServiceResponse<Trust> {
    match x_u_res_db_or_sr!(
        trust_dao::get_trust_with_both_ids(transaction, user_id, trust_req.trustee_id).await
    ) {
        Some(_) => {
            return service_error!(400, SE::NotFoundError("You already trust this player.".into()))
        }
        None => { /* Do nothing */ }
    };
    match x_u_res_db_or_sr!(get_player_with_id(transaction, user_id).await) {
        Some(player) => player,
        None => {
            return service_error!(404, SE::NotFoundError("Truster Player profile not found.".into()))
        }
    };
    match x_u_res_db_or_sr!(
        get_player_with_id(transaction, trust_req.trustee_id).await
    ) {
        Some(player) => player,
        None => {
            return service_error!(404, SE::NotFoundError("Trustee Player profile not found.".into()))
        }
    };
    let trust_to_insert = Trust::from_trust_dto(trust_req.clone(), user_id);
    if user_id == trust_req.trustee_id {
        return service_error!(400, SE::NotAllowed("You can't trust yourself...".into()));
    }
    Ok(x_u_res_db_or_sr!(trust_dao::insert_trust(transaction, &trust_to_insert).await))
}

pub async fn remove_trusted_player(
    transaction: &mut PgConnection,
    trust_req: TrustRequestDto,
    user_id: i32,
) -> ServiceResponse<Trust> {
    match x_u_res_db_or_sr!(get_player_with_id(transaction, user_id).await) {
        Some(player) => player,
        None => {
            return service_error!(404, SE::NotFoundError("Truster Player profile not found.".into()))
        }
    };
    match x_u_res_db_or_sr!(
        get_player_with_id(transaction, trust_req.trustee_id).await
    ) {
        Some(player) => player,
        None => {
            return service_error!(404, SE::NotFoundError("Trustee Player profile not found.".into()))
        }
    };
    Ok(x_u_res_db_or_sr!(
        trust_dao::delete_trust_with_both_ids(transaction, user_id, trust_req.trustee_id)
            .await
    ))
}
