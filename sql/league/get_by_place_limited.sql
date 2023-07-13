SELECT * FROM league
WHERE place_id = $1 AND visibility = 'Unlisted'
ORDER BY time_created DESC
LIMIT $2 OFFSET 25;