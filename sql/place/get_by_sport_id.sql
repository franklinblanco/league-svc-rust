SELECT * FROM place
WHERE sport_id = $1
ORDER BY time_created DESC
LIMIT 25 OFFSET $2;