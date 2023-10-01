INSERT INTO field (
    place_id,
    time_created,
    last_updated,
    country,
    city,
    name,
    price_per_hour,
    currency,
    description
) VALUES (
    $1,
    $2,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8
) RETURNING *;