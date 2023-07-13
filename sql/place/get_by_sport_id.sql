SELECT * FROM place
WHERE sport_id = $1
ORDER BY time_created DESC
LIMIT $2 OFFSET 25;