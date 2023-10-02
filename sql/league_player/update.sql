UPDATE league_player set 
last_updated = $1,
status = $2
WHERE id = $3 RETURNING 
    id,
    league_id,
    player_id,
    time_created,
    last_updated,
    status as "status: _";