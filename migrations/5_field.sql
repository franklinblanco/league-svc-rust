CREATE TABLE IF NOT EXISTS "field" (
    id SERIAL PRIMARY KEY,
    place_id INT NOT NULL,
    time_created TIMESTAMPTZ NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL,
    country VARCHAR(255) NOT NULL,
    city VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    price_per_hour FLOAT8 NOT NULL,
    currency VARCHAR(20) NOT NULL,
    description VARCHAR(2048)
);