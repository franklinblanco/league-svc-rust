-- Add migration script here
CREATE TABLE IF NOT EXISTS trust (
    id INT AUTO_INCREMENT PRIMARY KEY,
    truster_id INT NOT NULL,
    trustee_id INT NOT NULL,
    time_created DATETIME NOT NULL,
    last_updated DATETIME NOT NULL
);