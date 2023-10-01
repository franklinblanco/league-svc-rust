UPDATE field SET 
    place_id = $1,
    last_updated = $2,
    country = $3,
    city = $4,
    name = $5,
    price_per_hour = $6,
    currency = $7,
    description = $8
WHERE id = $9 RETURNING *;