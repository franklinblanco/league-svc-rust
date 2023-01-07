UPDATE league_player set 
last_updated = NOW(),
status = ?
WHERE id = ?;