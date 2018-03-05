-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    room_id INT REFERENCES rooms (id),
    name TEXT NOT NULL,
    email TEXT,
    is_admin BOOLEAN NOT NULL DEFAULT 'f'
)