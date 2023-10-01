INSERT INTO place (
    time_created,
    last_updated,
    name,
    sport_id,
    country,
    state,
    city,
    address,
    maps_url,
    contact_number,
    picture_url
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
) RETURNING *;