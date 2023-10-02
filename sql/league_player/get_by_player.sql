SELECT id,
    league_id,
    player_id,
    time_created,
    last_updated,
    status as "status: _" FROM league_player
WHERE player_id = $1;