CREATE TABLE IF NOT EXISTS "player" (
    id SERIAL PRIMARY KEY,
    time_created TIMESTAMPTZ NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL,
    name VARCHAR(255) NOT NULL,
    birth_date DATE NOT NULL,
    country VARCHAR(255) NOT NULL,
    city VARCHAR(255) NOT NULL,
    identification_number VARCHAR(255),
    bio VARCHAR(2048),
    profile_picture_url VARCHAR(1024),
    id_verified BOOLEAN NOT NULL,
    phone_number_verified BOOLEAN NOT NULL
);