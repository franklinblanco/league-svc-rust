SELECT * FROM place
WHERE country = $1
ORDER BY time_created DESC
LIMIT 25 OFFSET $2;