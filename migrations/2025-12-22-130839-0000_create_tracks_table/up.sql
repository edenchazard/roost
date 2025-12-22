-- Your SQL goes here
CREATE TABLE tracks (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    album VARCHAR(255) NOT NULL,
    artist VARCHAR(255) NOT NULL,
    track_number INT NOT NULL,
    picture_url VARCHAR(512)
);