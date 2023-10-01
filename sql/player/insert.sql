INSERT INTO player (
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
) VALUES (
    $1,
    $1, 
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9,
    $10
)
RETURNING *;