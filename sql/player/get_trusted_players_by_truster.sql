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
    p.id_verified as "id_verified: _",
    p.phone_number_verified as "phone_number_verified: _"
FROM trust t
INNER JOIN player p ON t.truster_id = p.id
WHERE p.id = ?
ORDER BY time_created DESC;