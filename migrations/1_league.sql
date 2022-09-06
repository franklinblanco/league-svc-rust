CREATE TABLE IF NOT EXISTS league (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    owner_id INT UNSIGNED NOT NULL,
    sport_id INT UNSIGNED NOT NULL,
    place_id INT UNSIGNED NOT NULL,
    time_created DATETIME NOT NULL,
    last_updated DATETIME NOT NULL,
    state VARCHAR(255) NOT NULL,
    visibility VARCHAR(255) NOT NULL,
    date_and_time DATETIME NOT NULL,
    cost_to_join DECIMAL(10, 2) NOT NULL,
    currency VARCHAR(20),
    max_players INT UNSIGNED NOT NULL,
    description VARCHAR(2048)
);