UPDATE player SET
    last_updated = NOW(),
    name = ?,
    birth_date = ?,
    country = ?,
    city = ?,
    identification_number = ?,
    bio = ?,
    profile_picture_url = ?,
    id_verified = ?,
    phone_number_verified = ?
WHERE id = ?;