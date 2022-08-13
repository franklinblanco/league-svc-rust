UPDATE place SET
    last_updated = NOW(),
    name = ?,
    sport_id = ?,
    address = ?,
    maps_url = ?,
    contact_number = ?,
    picture_url = ?
WHERE id = ?