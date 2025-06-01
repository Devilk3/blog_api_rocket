-- Your SQL goes here
CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    first_name TEXT,
    last_name TEXT
);

CREATE TABLE posts (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    created_by INTEGER NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    FOREIGN KEY(created_by) REFERENCES users(id)
);