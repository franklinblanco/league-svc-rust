UPDATE league_player set 
last_updated = $1,
status = $2
WHERE id = $3 RETURNING *;