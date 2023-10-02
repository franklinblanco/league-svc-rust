use actix_web_utils::{extensions::typed_response::TypedResponse, ServiceResponse, x_u_res_db_or_sr, service_error, x_u_res_or_sr};
use err::{MessageResource, Error, trace, ErrorType, ServiceError as SE};
use league_types::{
    domain::{
        enums::league_player_status::{ApprovalStatus, LeaguePlayerStatus, StatusType},
        league::{League, LeagueVisibility},
        league_player::LeaguePlayer,
        player::Player,
    },
    dto::league_player::JoinRequest,
};
use reqwest::Client;
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
    conn: &mut PgConnection,
    client: &Client,
    join_req: JoinRequest,
    user_id: i32,
) -> ServiceResponse<LeaguePlayer> {
    let mut transaction = x_u_res_db_or_sr!(conn.begin().await);
    let user_for_auth = UserForAuthenticationDto {
        app: APP_NAME.to_owned(),
        id: join_req.user_id.to_string(),
        token: join_req.auth_token.clone(),
    };
    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        LeaguePlayer
    );
    match unwrap_or_return_handled_error!(
        league_player_dao::get_league_players_by_player_id_and_league_id(
            conn,
            join_req.league_id,
            user.id as i32
        )
        .await,
        LeaguePlayer
    )
    .get(0)
    {
        Some(league_player) => {
            TypedResponse::return_standard_response(200, league_player.clone())
        }
        None => TypedResponse::return_standard_error(
            404,
            MessageResource::new_from_str("LeaguePlayer not found with given ids."),
        ),
    }
}

/// This method is called by the owner of the league to accept or deny
/// league players.
pub async fn change_league_request_status(
    conn: &mut PgConnection,
    client: &Client,
    new_status: ApprovalStatus,
    join_req: JoinRequest,
    user_id: i32,
) -> ServiceResponse<LeaguePlayer> {
    let mut transaction = x_u_res_db_or_sr!(conn.begin().await);
    let league = match unwrap_or_return_handled_error!(
        league_dao::get_league_with_id(conn, join_req.league_id).await,
        LeaguePlayer
    ) {
        Some(league) => league,
        None => {
            return TypedResponse::return_standard_error(
                404,
                MessageResource::new_from_str("League not found with given id."),
            )
        }
    };
    let user_for_auth = UserForAuthenticationDto {
        app: APP_NAME.to_owned(),
        id: league.owner_id.to_string(),
        token: join_req.auth_token.clone(),
    };
    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        LeaguePlayer
    );
    if league.owner_id != user.id as i32 {
        return TypedResponse::return_standard_error(
            401,
            MessageResource::new_from_str("You don't own this league..."),
        );
    }
    let persisted_league_players = unwrap_or_return_handled_error!(
        league_player_dao::get_league_players_by_player_id_and_league_id(
            conn,
            join_req.league_id,
            join_req.user_id
        )
        .await,
        LeaguePlayer
    );

    if persisted_league_players.is_empty() {
        return TypedResponse::return_standard_error(
            404,
            MessageResource::new_from_str("No LeaguePlayer found with given ids."),
        );
    }

    match attempt_league_request_status_change(&persisted_league_players, new_status) {
        Ok(new_status) => {
            let league_player_to_persist = persisted_league_players.get(new_status.1).unwrap(); // Dangerous unwrap but should always work as this comes from iterating the vec
            unwrap_or_return_handled_error!(
                league_player_dao::update_league_player_status(
                    conn,
                    league_player_to_persist.id,
                    &new_status.0
                )
                .await,
                LeaguePlayer
            );
            return TypedResponse::return_standard_response(
                200,
                unwrap_or_return_handled_error!(
                    league_player_dao::get_league_player_by_id(conn, league_player_to_persist.id)
                        .await,
                    LeaguePlayer
                ),
            );
        }
        Err(err) => return err,
    };
}

pub async fn get_all_leagues_player_has_applied_to(
    conn: &mut PgConnection,
    client: &Client,
    join_req: JoinRequest,
    page: i64,
    user_id: i32,
) -> ServiceResponse<Vec<League>> {
    let mut transaction = x_u_res_db_or_sr!(conn.begin().await);
    let user_for_auth = UserForAuthenticationDto {
        app: APP_NAME.to_owned(),
        id: join_req.user_id.to_string(),
        token: join_req.auth_token.clone(),
    };
    unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        Vec<League>
    );
    let resulting_leagues = unwrap_or_return_handled_error!(
        league_dao::get_all_leagues_player_has_applied_to(
            conn,
            join_req.user_id,
            page
        )
        .await,
        Vec<League>
    );
    if resulting_leagues.len() > 0 {
        return TypedResponse::return_standard_response(200, resulting_leagues);
    }
    return TypedResponse::return_standard_error(
        404,
        MessageResource::new_from_str("No leagues found with player join requests."),
    );
}

pub async fn get_all_players_in_league(
    conn: &mut PgConnection,
    client: &Client,
    join_req: JoinRequest,
    user_id: i32,
) -> ServiceResponse<Vec<Player>> {
    let mut transaction = x_u_res_db_or_sr!(conn.begin().await);
    let user_for_auth = UserForAuthenticationDto {
        app: APP_NAME.to_owned(),
        id: join_req.user_id.to_string(),
        token: join_req.auth_token.clone(),
    };
    unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        Vec<Player>
    );
    let resulting_players: Vec<Player> = unwrap_or_return_handled_error!(
        player_dao::get_all_players_in_league(conn, join_req.league_id).await,
        Vec<Player>
    )
    .into_iter()
    .map(|player| player.clear_all_sensitive_fields())
    .collect();
    if resulting_players.len() > 0 {
        return TypedResponse::return_standard_response(200, resulting_players);
    }
    return TypedResponse::return_standard_error(
        404,
        MessageResource::new_from_str(
            "No players found with join requests to the league specified.",
        ),
    );
}

pub async fn leave_league(
    conn: &mut PgConnection,
    client: &Client,
    join_req: JoinRequest,
    user_id: i32,
) -> ServiceResponse<LeaguePlayer> {
    let mut transaction = x_u_res_db_or_sr!(conn.begin().await);
    let user_for_auth = UserForAuthenticationDto {
        app: APP_NAME.to_owned(),
        id: join_req.user_id.to_string(),
        token: join_req.auth_token.clone(),
    };
    unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        LeaguePlayer
    );
    let league_players = unwrap_or_return_handled_error!(
        league_player_dao::get_league_players_by_player_id_and_league_id(
            &conn,
            join_req.league_id,
            join_req.user_id
        )
        .await,
        LeaguePlayer
    );
    // Just check if there is an active league_player
    for (_index, league_player) in league_players.iter().enumerate() {
        let status = unwrap_or_return_handled_error!(
            LeaguePlayerStatus::from_str(league_player.status.as_str()),
            LeaguePlayer
        );
        if status.get_status_type() == StatusType::Active {
            //
            let status_to_be_persisted = match status {
                LeaguePlayerStatus::Joined => LeaguePlayerStatus::Left,
                LeaguePlayerStatus::Requested => LeaguePlayerStatus::Canceled,
                LeaguePlayerStatus::Invited => LeaguePlayerStatus::Canceled,
                _ => {
                    return TypedResponse::return_standard_error(
                        500,
                        MessageResource::new_from_str("Something went wrong."),
                    )
                }
            };
            let updated_league_player = unwrap_or_return_handled_error!(
                league_player_dao::update_league_player_status(
                    conn,
                    league_player.id,
                    &status_to_be_persisted
                )
                .await,
                LeaguePlayer
            );
            return TypedResponse::return_standard_response(200, updated_league_player);
        }
    }
    println!(
        "Player tried to leave without having active leagues... LeaguePlayers: {:#?}",
        league_players
    );
    return TypedResponse::return_standard_error(
        404,
        MessageResource::new_from_str(
            "No players found with active join requests to the league specified.",
        ),
    );
}

// #################
// PRIVATE FUNCTIONS
// #################

fn attempt_league_request_status_change(
    persisted_league_players: &Vec<LeaguePlayer>,
    new_status: ApprovalStatus,
) -> Result<(LeaguePlayerStatus, usize), ServiceResponse<LeaguePlayer>> {
    let mut last_error: ServiceResponse<LeaguePlayer> =
        TypedResponse::return_empty_response(400);

    for (index, persisted_league_player) in persisted_league_players.iter().enumerate() {
        let persisted_status = match persisted_league_player.status.parse::<LeaguePlayerStatus>() {
            Ok(status) => status,
            Err(err) => return Err(err.convert_to_returnable(400)),
        };
        match new_status {
            ApprovalStatus::Approved => {
                if persisted_status == LeaguePlayerStatus::Requested {
                    return Ok((LeaguePlayerStatus::Joined, index));
                } else {
                    last_error = TypedResponse::return_standard_error(
                        400,
                        MessageResource::new_from_str(
                            "Cannot approve LeaguePlayer with non-approvable status.",
                        ),
                    );
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
                        last_error = TypedResponse::return_standard_error(
                            400,
                            MessageResource::new_from_str(
                                "Cannot deny LeaguePlayer with non-deniable status.",
                            ),
                        );
                    }
                };
            }
        }
    }
    Err(last_error)
}
