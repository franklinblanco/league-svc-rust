CREATE TABLE IF NOT EXISTS field (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    place_id INT UNSIGNED NOT NULL,
    time_created DATETIME NOT NULL,
    last_updated DATETIME NOT NULL,
    country VARCHAR(255) NOT NULL,
    city VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    price_per_hour DECIMAL(10, 2) NOT NULL,
    currency VARCHAR(20) NOT NULL,
    description VARCHAR(2048)
);