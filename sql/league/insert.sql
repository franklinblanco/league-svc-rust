INSERT INTO league (
    id,
    owner_id,
    sport_id,
    time_created,
    last_updated,
    state,
    visibility,
    date_and_time,
    cost_to_join,
    currency,
    max_players,
    description
) VALUES(
    NULL,
    ?,
    ?,
    NOW(),
    NOW(),
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)