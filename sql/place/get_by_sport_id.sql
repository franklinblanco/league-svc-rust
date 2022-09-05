SELECT * FROM place
WHERE sport_id = ?
ORDER BY time_created DESC
LIMIT ?,?;