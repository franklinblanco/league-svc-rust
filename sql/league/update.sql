UPDATE league SET 
    owner_id = $1,
    sport_id = $2,
    place_id = $3,
    last_updated = $4,
    state = $5,
    visibility = $6,
    date_and_time = $7,
    cost_to_join = $8,
    currency = $9,
    max_players = $10,
    description = $11
WHERE id = $12 RETURNING 
    id,
    owner_id,
    sport_id,
    place_id,
    time_created,
    last_updated,
    state as "state: _",
    visibility as "visibility: _",
    date_and_time,
    cost_to_join,
    currency,
    max_players,
    description;