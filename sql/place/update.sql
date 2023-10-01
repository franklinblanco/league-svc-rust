UPDATE place SET
    last_updated = $1,
    name = $2,
    sport_id = $3,
    country = $4,
    state = $5,
    city = $6,
    address = $7,
    maps_url = $8,
    contact_number = $9,
    picture_url = $10
WHERE id = $11 RETURNING *;