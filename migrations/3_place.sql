CREATE TABLE IF NOT EXISTS "place" (
    id SERIAL PRIMARY KEY,
    time_created TIMESTAMPTZ NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL,
    name VARCHAR(255) NOT NULL,
    sport_id INT NOT NULL,
    country VARCHAR(255) NOT NULL,
    state VARCHAR(255),
    city VARCHAR(255) NOT NULL,
    address VARCHAR(1024) NOT NULL,
    maps_url VARCHAR(1024),
    contact_number VARCHAR(11),
    picture_url VARCHAR(1024)
);