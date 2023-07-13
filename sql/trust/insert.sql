INSERT INTO trust (
    truster_id,
    trustee_id,
    time_created,
    last_updated
) VALUES (
    $1,
    $2,
    $3,
    $3
);