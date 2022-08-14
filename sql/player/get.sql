SELECT
    id,
    time_created,
    last_updated,
    name,
    birth_date,
    country,
    city,
    identification_number,
    bio,
    profile_picture_url,
    id_verified as "id_verified: _",
    phone_number_verified as "phone_number_verified: _"
FROM player
WHERE id = ?;