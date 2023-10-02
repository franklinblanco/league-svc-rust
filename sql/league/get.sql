SELECT id,
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
    description FROM league
WHERE id = $1;