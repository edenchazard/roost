-- Your SQL goes here
CREATE TABLE albums (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    title VARCHAR(255),
    artist VARCHAR(255),
    picture_url VARCHAR(512),
    UNIQUE (title, artist)
);
