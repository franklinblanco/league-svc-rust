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
    id_verified,
    phone_number_verified
FROM player
WHERE id = $1;