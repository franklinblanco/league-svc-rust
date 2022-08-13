INSERT INTO place (
    time_created,
    last_updated,
    name,
    sport_id,
    address,
    maps_url,
    contact_number,
    picture_url
) VALUES (
    NOW(),
    NOW(),
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
);