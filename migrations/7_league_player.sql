CREATE TABLE IF NOT EXISTS "league_player" (
    id SERIAL PRIMARY KEY,
    league_id INT NOT NULL,
    player_id INT NOT NULL,
    time_created TIMESTAMPTZ NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL,
    status VARCHAR(255) NOT NULL
);