CREATE TABLE IF NOT EXISTS league (
    id INT AUTO_INCREMENT PRIMARY KEY,
    owner_id INT NOT NULL,
    sport_id INT NOT NULL,
    time_created DATETIME NOT NULL,
    last_updated DATETIME NOT NULL,
    visibility VARCHAR(255) NOT NULL,
    date_and_time DATETIME NOT NULL,
    cost_to_join DECIMAL(10, 2) NOT NULL,
    currency VARCHAR(20) NOT NULL,
    max_players INT NOT NULL,
    description VARCHAR(2048)
);