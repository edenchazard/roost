-- Your SQL goes here
CREATE TABLE albums (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255),
    artist VARCHAR(255),
    picture_url VARCHAR(512)
);