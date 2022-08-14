UPDATE field SET 
    place_id = ?,
    last_updated = NOW(),
    country = ?,
    city = ?,
    name = ?,
    price_per_hour = ?,
    currency = ?,
    description = ?
WHERE id = ?