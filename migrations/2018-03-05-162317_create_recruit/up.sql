-- Your SQL goes here
CREATE TABLE recruits (
    id SERIAL PRIMARY KEY,
    start_at TIMESTAMP NOT NULL,
    end_at TIMESTAMP NOT NULL
);
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    recruit_id INT NOT NULL REFERENCES recruits (id),
    contribution INT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    start_at TIMESTAMP NOT NULL,
    end_at TIMESTAMP NOT NULL
);
CREATE TABLE assignments (
    id SERIAL PRIMARY KEY,
    task_id INT NOT NULL REFERENCES tasks (id),
    user_id INT NOT NULL REFERENCES users (id)
);
CREATE TABLE entries (
    id SERIAL PRIMARY KEY,
    task_id INT NOT NULL REFERENCES tasks (id),
    user_id INT NOT NULL REFERENCES users (id)
);