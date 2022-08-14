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
    ?,
    NOW(),
    NOW(),
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
);