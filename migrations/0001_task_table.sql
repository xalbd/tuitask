CREATE TABLE task (
    id SERIAL PRIMARY KEY,
    name varchar NOT NULL,
    due_date date NOT NULL,
    completed boolean DEFAULT FALSE
)