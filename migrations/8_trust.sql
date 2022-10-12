-- Add migration script here
CREATE TABLE IF NOT EXISTS trust (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    truster_id INT UNSIGNED NOT NULL,
    trustee_id INT UNSIGNED NOT NULL,
    time_created TIMESTAMP NOT NULL,
    last_updated TIMESTAMP NOT NULL
);