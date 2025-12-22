-- Your SQL goes here
CREATE TABLE tracks (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255),
    album VARCHAR(255),
    artist VARCHAR(255),
    track_number INT,
    picture_url VARCHAR(512),
    audio_url VARCHAR(512)
);