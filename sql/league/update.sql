UPDATE league SET 
    owner_id = ?,
    sport_id = ?,
    time_created = NOW(),
    last_updated = NOW(),
    state = ?,
    visibility = ?,
    date_and_time = ?,
    cost_to_join = ?,
    currency = ?,
    max_players = ?,
    description = ?
WHERE id = ?;