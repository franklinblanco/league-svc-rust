CREATE TABLE IF NOT EXISTS "user" (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  password TEXT NOT NULL,
  salt TEXT NOT NULL,
  time_created TIMESTAMPTZ NOT NULL,
  last_updated TIMESTAMPTZ NOT NULL
);