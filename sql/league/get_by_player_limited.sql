SELECT * FROM league
WHERE owner_id = ? AND visibility <>?
ORDER BY time_created DESC
LIMIT ?,?;