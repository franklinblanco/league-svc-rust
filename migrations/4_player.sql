CREATE TABLE IF NOT EXISTS player (
    id INT AUTO_INCREMENT PRIMARY KEY,
    time_created DATETIME NOT NULL,
    last_updated DATETIME NOT NULL,
    name VARCHAR(255) NOT NULL,
    birth_date DATETIME NOT NULL,
    bio VARCHAR(2048),
    profile_picture_url VARCHAR(1024),
    id_verified TINYINT(1) NOT NULL,
    phone_number_verified TINYINT(1) NOT NULL
);