CREATE TABLE IF NOT EXISTS "trust" (
    id SERIAL PRIMARY KEY,
    truster_id INT NOT NULL,
    trustee_id INT NOT NULL,
    time_created TIMESTAMPTZ NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL
);