SELECT * FROM place
WHERE country = ?
ORDER BY time_created DESC
LIMIT ?,?;