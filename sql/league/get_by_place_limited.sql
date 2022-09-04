SELECT * FROM league
WHERE place_id = ? AND visibility <>"Unlisted"
ORDER BY time_created DESC
LIMIT ?,?;