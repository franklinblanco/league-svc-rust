INSERT INTO league (
    id,
    owner_id,
    sport_id,
    place_id,
    time_created,
    last_updated,
    state,
    visibility,
    date_and_time,
    cost_to_join,
    currency,
    max_players,
    description
) VALUES (
    NULL,
    $1,
    $2,
    $3,
    $4,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9,
    $10,
    $11
) RETURNING *;