use actix_web_utils::{ServiceResponse, x_u_res_db_or_sr, service_error};
use err::{ServiceError as SE, VecRemove};
use league_types::{
    domain::{
        enums::league_player_status::{ApprovalStatus, LeaguePlayerStatus, StatusType},
        league::{League, LeagueVisibility},
        league_player::LeaguePlayer,
        player::Player,
    },
    dto::league_player::JoinRequest,
};
use sqlx::PgConnection;

use crate::dao::{league_dao, league_player_dao, player_dao, trust_dao};

/// Creates a LeaguePlayer and checks if the league is open or closed
pub async fn request_to_join_league(
    transaction: &mut PgConnection,
    join_req: JoinRequest,
    user_id: i32,
) -> ServiceResponse<LeaguePlayer> {
    // Get league
    let league = match x_u_res_db_or_sr!(
        league_dao::get_league_with_id(transaction, join_req.league_id).await
    ) {
        Some(league) => league,
        None => return service_error!(404, SE::NotFoundError("No league found with that ID".into())),
    };
    // Get Player profile
    match x_u_res_db_or_sr!(
        player_dao::get_player_with_id(transaction, user_id).await
    ) {
        Some(player) => player,
        None => return service_error!(404, SE::NotFoundError("Player profile not found.".into()))
    };
    // Build LeaguePlayer
    let mut league_player_to_insert = LeaguePlayer::from_join_req(join_req, user_id);
    // Get existing league players if any
    let persisted_league_players = x_u_res_db_or_sr!(
        league_player_dao::get_league_players_by_player_id_and_league_id(
            transaction,
            league_player_to_insert.league_id,
            league_player_to_insert.player_id
        )
        .await
    );
    let mut player_has_inactive_persisted_league_players: bool = false;
    // Check if there are league players
    if !persisted_league_players.is_empty() {
        // Loop through the persisted LeaguePlayers
        // And check if there are any active ones.
        for persisted_league_player in persisted_league_players {
            // Parse league player status into enum
            let persisted_league_player_status = persisted_league_player.status;
            if persisted_league_player_status.get_status_type() == StatusType::Active {
                return service_error!(400, SE::AlreadyExistsError("You already have an active join request for this league.".into()));
            }
            player_has_inactive_persisted_league_players = true;
        }
    }
    // Match LeagueVisibility
    // Public -> Join (If player hasn't left before) Private -> Trust model
    // Unlisted will deny a player right away
    let join_request_status = match league.visibility {
        // If player has previous inactive LeaguePlayers then don't allow a rejoin.
        LeagueVisibility::Public => match player_has_inactive_persisted_league_players {
            true => return service_error!(400, SE::AlreadyExistsError("Player has already left or been kicked out of this league.".into())),
            false => LeaguePlayerStatus::Joined,
        },
        // If player is trusted then Join the league.
        // If player isn't trusted then request to join the league.
        LeagueVisibility::Private => match x_u_res_db_or_sr!(
            trust_dao::get_trust_with_both_ids(
                transaction,
                league.owner_id,
                league_player_to_insert.player_id
            )
            .await
        ) {
            Some(_) => LeaguePlayerStatus::Joined,
            None => LeaguePlayerStatus::Requested,
        },
        LeagueVisibility::Unlisted => LeaguePlayerStatus::Denied,
    };
    // Insert league_player_status into DB
    league_player_to_insert.status = join_request_status;
    let persisted_league_player = x_u_res_db_or_sr!(
        league_player_dao::insert_league_player(transaction, &league_player_to_insert).await
    );
    // Return both cases, the ResultingLeaguePlayer
    Ok(persisted_league_player)
}

pub async fn get_league_request_status(
    transaction: &mut PgConnection,
    join_req: JoinRequest,
    user_id: i32,
) -> ServiceResponse<LeaguePlayer> {
    match x_u_res_db_or_sr!(
        league_player_dao::get_league_players_by_player_id_and_league_id(
            transaction,
            join_req.league_id,
            user_id
        )
        .await
    )
    .try_remove(0)
    {
        Some(league_player) => {
            Ok(league_player)
        }
        None => service_error!(404, SE::NotFoundError("LeaguePlayer not found with given ids.".into()))
    }
}

/// This method is called by the owner of the league to accept or deny
/// league players.
pub async fn change_league_request_status(
    transaction: &mut PgConnection,
    new_status: ApprovalStatus,
    join_req: JoinRequest,
    user_id: i32,
) -> ServiceResponse<LeaguePlayer> {
    let league = match x_u_res_db_or_sr!(
        league_dao::get_league_with_id(transaction, join_req.league_id).await
    ) {
        Some(league) => league,
        None => return service_error!(404, SE::NotFoundError("League not found with given id.".into()))
    };
    if league.owner_id != user_id {
        return service_error!(404, SE::NotFoundError("You don't own this league...".into()));
    }
    let persisted_league_players = x_u_res_db_or_sr!(
        league_player_dao::get_league_players_by_player_id_and_league_id(
            transaction,
            join_req.league_id,
            user_id
        )
        .await
    );

    if persisted_league_players.is_empty() {
        return service_error!(404, SE::NotFoundError("No LeaguePlayer found with given ids.".into()));
    }

    match attempt_league_request_status_change(&persisted_league_players, new_status) {
        Ok(new_status) => {
            let league_player_to_persist = persisted_league_players.get(new_status.1).unwrap(); // Dangerous unwrap but should always work as this comes from iterating the vec
            x_u_res_db_or_sr!(
                league_player_dao::update_league_player_status(
                    transaction,
                    league_player_to_persist.id,
                    &new_status.0
                )
                .await
            );
            return Ok(x_u_res_db_or_sr!(league_player_dao::get_league_player_by_id(transaction, league_player_to_persist.id).await));
        }
        Err(err) => return err,
    };
}

pub async fn get_all_leagues_player_has_applied_to(
    transaction: &mut PgConnection,
    page: i64,
    user_id: i32,
) -> ServiceResponse<Vec<League>> {
    let resulting_leagues = x_u_res_db_or_sr!(
        league_dao::get_all_leagues_player_has_applied_to(
            transaction,
            user_id,
            page
        )
        .await
    );
    if resulting_leagues.len() > 0 {
        return Ok(resulting_leagues);
    }
    service_error!(404, SE::NotFoundError("No leagues found with player join requests.".into()))
}

pub async fn get_all_players_in_league(
    transaction: &mut PgConnection,
    join_req: JoinRequest,
    _user_id: i32,
) -> ServiceResponse<Vec<Player>> {
    let resulting_players: Vec<Player> = x_u_res_db_or_sr!(
        player_dao::get_all_players_in_league(transaction, join_req.league_id, LeaguePlayerStatus::Joined).await
    )
    .into_iter()
    .map(|player| player.clear_all_sensitive_fields())
    .collect();
    if resulting_players.len() > 0 {
        return Ok(resulting_players);
    }
    service_error!(
        404,
        SE::NotFoundError(
            "No players found with join requests to the league specified.".into()
        )
    )
}

pub async fn leave_league(
    transaction: &mut PgConnection,
    join_req: JoinRequest,
    user_id: i32,
) -> ServiceResponse<LeaguePlayer> {
    let league_players = x_u_res_db_or_sr!(
        league_player_dao::get_league_players_by_player_id_and_league_id(
            transaction,
            join_req.league_id,
            user_id
        )
        .await
    );
    // Just check if there is an active league_player
    for (_index, league_player) in league_players.iter().enumerate() {
        if league_player.status.get_status_type() == StatusType::Active {
            let status_to_be_persisted = match league_player.status {
                LeaguePlayerStatus::Joined => LeaguePlayerStatus::Left,
                LeaguePlayerStatus::Requested => LeaguePlayerStatus::Canceled,
                LeaguePlayerStatus::Invited => LeaguePlayerStatus::Canceled,
                _ => {
                    return service_error!(
                        500,
                        SE::UnexpectedError("Something went wrong.".into())
                    )
                }
            };
            let updated_league_player = x_u_res_db_or_sr!(
                league_player_dao::update_league_player_status(
                    transaction,
                    league_player.id,
                    &status_to_be_persisted
                )
                .await
            );
            return Ok(updated_league_player);
        }
    }
    log::warn!(
        "Player tried to leave without having active leagues... LeaguePlayers: {:#?}",
        league_players
    );
    service_error!(404, SE::NotFoundError("No players found with active join requests to the league specified.".into()))
}

// #################
// PRIVATE FUNCTIONS
// #################

/// TODO: Test and validate this function. Wtf old me?
/// Hint: Vec of good things and vec of bad things, not whatever happens first...
fn attempt_league_request_status_change(
    persisted_league_players: &Vec<LeaguePlayer>,
    new_status: ApprovalStatus,
) -> Result<(LeaguePlayerStatus, usize), ServiceResponse<LeaguePlayer>> {
    let mut last_error: ServiceResponse<LeaguePlayer> = service_error!(400, SE::NotAllowed("No league players.".into()));

    for (index, persisted_league_player) in persisted_league_players.iter().enumerate() {
        let persisted_status = &persisted_league_player.status;
        match new_status {
            ApprovalStatus::Approved => {
                if persisted_status == &LeaguePlayerStatus::Requested {
                    return Ok((LeaguePlayerStatus::Joined, index));
                } else {
                    last_error = service_error!(400, SE::NotAllowed("Cannot approve LeaguePlayer with non-approvable status.".into()));
                }
            }
            ApprovalStatus::Denied => {
                match persisted_status {
                    LeaguePlayerStatus::Joined => return Ok((LeaguePlayerStatus::Kicked, index)),
                    LeaguePlayerStatus::Requested => {
                        return Ok((LeaguePlayerStatus::Denied, index))
                    }
                    LeaguePlayerStatus::Invited => return Ok((LeaguePlayerStatus::Denied, index)),
                    _ => {
                        last_error = service_error!(400, SE::NotAllowed("Cannot deny LeaguePlayer with non-deniable status.".into())
                        );
                    }
                };
            }
        }
    }
    Err(last_error)
}
