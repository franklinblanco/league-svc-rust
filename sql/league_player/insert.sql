INSERT INTO league_player (
    league_id,
    player_id,
    time_created,
    last_updated,
    status
) VALUES (
    ?,
    ?,
    NOW(),
    NOW(),
    ?
);