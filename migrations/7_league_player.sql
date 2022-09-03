CREATE TABLE IF NOT EXISTS league_player (
    id INT AUTO_INCREMENT PRIMARY KEY,
    league_id INT NOT NULL,
    player_id INT NOT NULL,
    time_created DATETIME NOT NULL,
    last_updated DATETIME NOT NULL,
    status VARCHAR(255) NOT NULL
);