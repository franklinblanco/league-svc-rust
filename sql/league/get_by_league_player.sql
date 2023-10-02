SELECT le.id,
    le.owner_id,
    le.sport_id,
    le.place_id,
    le.time_created,
    le.last_updated,
    le.state as "state: _",
    le.visibility as "visibility: _",
    le.date_and_time,
    le.cost_to_join,
    le.currency,
    le.max_players,
    le.description
FROM league le
INNER JOIN league_player lp ON
lp.league_id = le.id
WHERE lp.player_id = $1
ORDER BY le.time_created DESC
LIMIT $2 OFFSET 25;