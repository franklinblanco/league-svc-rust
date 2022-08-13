CREATE TABLE IF NOT EXISTS place (
    id INT AUTO_INCREMENT PRIMARY KEY,
    time_created DATETIME NOT NULL,
    last_updated DATETIME NOT NULL,
    name VARCHAR(255) NOT NULL,
    sport_id INT NOT NULL,
    address VARCHAR(1024),
    maps_url VARCHAR(1024),
    contact_number VARCHAR(11),
    picture_url VARCHAR(1024)
);