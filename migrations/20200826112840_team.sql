-- Add migration script here
CREATE TABLE IF NOT EXISTS team (
    name varchar(15) PRIMARY KEY,
    role varchar(30) NOT NULL,
    age integer NOT NULL CHECK (age > 0)
);