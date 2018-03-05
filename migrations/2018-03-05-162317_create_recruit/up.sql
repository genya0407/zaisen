-- Your SQL goes here
CREATE TABLE recruits (
    id SERIAL PRIMARY KEY,
    start_at TIMESTAMP NOT NULL,
    end_at TIMESTAMP NOT NULL
);
CREATE TABLE recruit_tasks (
    id SERIAL PRIMARY KEY,
    recruit_id INT REFERENCES recruits (id),
    task_id INT REFERENCES tasks (id)
);
CREATE TABLE entries (
    id SERIAL PRIMARY KEY,
    recruit_task_id INT REFERENCES recruit_tasks (id),
    user_id INT REFERENCES users (id)
)