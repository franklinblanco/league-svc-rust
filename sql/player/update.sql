UPDATE player SET
    last_updated = $1,
    name = $2,
    birth_date = $3,
    country = $4,
    city = $5,
    identification_number = $6,
    bio = $7,
    profile_picture_url = $8,
    id_verified = $9,
    phone_number_verified = $10
WHERE id = $11;