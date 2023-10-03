SELECT l.id,
    l.owner_id,
    l.sport_id,
    l.place_id,
    l.time_created,
    l.last_updated,
    l.state as "state: _",
    l.visibility as "visibility: _",
    l.date_and_time,
    l.cost_to_join,
    l.currency,
    l.max_players,
    l.description FROM league l
LEFT JOIN place p ON p.id = l.place_id
WHERE p.country = $1 AND visibility != 'Unlisted'
ORDER BY l.time_created DESC
LIMIT 25 OFFSET $2;