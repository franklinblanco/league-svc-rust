SELECT l.* FROM league l
LEFT JOIN place p ON p.id = l.place_id
WHERE p.country = ?
ORDER BY l.time_created DESC
LIMIT ?