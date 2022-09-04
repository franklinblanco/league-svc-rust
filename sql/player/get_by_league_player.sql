SELECT     
    pl.id,
    pl.time_created,
    pl.last_updated,
    pl.name,
    pl.birth_date,
    pl.country,
    pl.city,
    pl.identification_number,
    pl.bio,
    pl.profile_picture_url,
    pl.id_verified as "id_verified: _",
    pl.phone_number_verified as "phone_number_verified: _"
FROM player pl
INNER JOIN league_player lp ON
lp.player_id = pl.id
WHERE lp.league_id = ?;