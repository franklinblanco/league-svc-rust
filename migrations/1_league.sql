CREATE TABLE IF NOT EXISTS "league" (
    id SERIAL PRIMARY KEY,
    owner_id INT NOT NULL,
    sport_id INT NOT NULL,
    place_id INT NOT NULL,
    time_created TIMESTAMPTZ NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL,
    state VARCHAR(255) NOT NULL,
    visibility VARCHAR(255) NOT NULL,
    date_and_time TIMESTAMPTZ NOT NULL,
    cost_to_join FLOAT8 NOT NULL,
    currency VARCHAR(20),
    max_players INT NOT NULL,
    description TEXT
);