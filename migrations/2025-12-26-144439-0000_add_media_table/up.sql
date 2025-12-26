-- Your SQL goes here
CREATE TABLE  media (
    id SERIAL PRIMARY KEY,
    path VARCHAR(255) UNIQUE NOT NULL,
    mime_type TEXT NOT NULL,
    type_ VARCHAR(50) NOT NULL,
    mediable_id INTEGER NOT NULL,
    mediable_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);