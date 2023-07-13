INSERT INTO league_player (
    league_id,
    player_id,
    time_created,
    last_updated,
    status
) VALUES (
    $1,
    $2,
    $3,
    $3,
    $4
) RETURNING *;