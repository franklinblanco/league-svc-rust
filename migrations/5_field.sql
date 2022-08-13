CREATE TABLE IF NOT EXISTS field (
    id INT AUTO_INCREMENT PRIMARY KEY,
    place_id INT NOT NULL,
    time_created DATETIME NOT NULL,
    last_updated DATETIME NOT NULL,
    name VARCHAR(255) NOT NULL,
    price_per_hour DECIMAL(10, 2) NOT NULL,
    description VARCHAR(2048)
);