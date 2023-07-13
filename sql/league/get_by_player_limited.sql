SELECT * FROM league
WHERE owner_id = $1 AND visibility = $2
ORDER BY time_created DESC
LIMIT $3 OFFSET 25;