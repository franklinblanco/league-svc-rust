SELECT l.* FROM league l
LEFT JOIN place p ON p.id = l.place_id
WHERE p.country = $1 AND visibility = 'Unlisted'
ORDER BY l.time_created DESC
LIMIT $2 OFFSET 25;