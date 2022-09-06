UPDATE league_player set 
last_updated = NOW(),
status = ?
WHERE league_id = ? AND player_id = ?;