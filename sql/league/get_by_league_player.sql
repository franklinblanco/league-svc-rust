SELECT le.*
FROM league le
INNER JOIN league_player lp ON
lp.league_id = le.id
WHERE lp.player_id = ?
ORDER BY le.time_created DESC
LIMIT ?,?;