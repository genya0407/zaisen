-- Your SQL goes here
CREATE TABLE recruits (
    id SERIAL PRIMARY KEY,
    start_at TIMESTAMP NOT NULL,
    end_at TIMESTAMP NOT NULL
);
CREATE TABLE recruit_tasks (
    id SERIAL PRIMARY KEY,
    recruit_id INT NOT NULL REFERENCES recruits (id),
    task_id INT NOT NULL REFERENCES tasks (id)
);
CREATE TABLE entries (
    id SERIAL PRIMARY KEY,
    task_id INT NOT NULL REFERENCES tasks (id),
    user_id INT NOT NULL REFERENCES users (id)
)