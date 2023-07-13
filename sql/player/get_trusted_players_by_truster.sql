SELECT p.id,
    p.time_created,
    p.last_updated,
    p.name,
    p.birth_date,
    p.country,
    p.city,
    p.identification_number,
    p.bio,
    p.profile_picture_url,
    id_verified,
    phone_number_verified
FROM trust t
INNER JOIN player p ON t.truster_id = p.id
WHERE p.id = $1
ORDER BY time_created DESC;