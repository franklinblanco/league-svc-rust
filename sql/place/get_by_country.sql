SELECT * FROM place
WHERE country = $1
ORDER BY time_created DESC
LIMIT $2 OFFSET 25;