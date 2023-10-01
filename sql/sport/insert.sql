INSERT INTO sport (
    id,
    name,
    category_id
) VALUES (
    $1,
    $2,
    $3
) RETURNING *;