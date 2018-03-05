-- Your SQL goes here
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    assignee INT REFERENCES users (id),
    contribution INT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    start_at TIMESTAMP NOT NULL,
    end_at TIMESTAMP NOT NULL
)