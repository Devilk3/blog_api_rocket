-- Your SQL goes here
CREATE TABLE posts_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    fk_post_id INTEGER NOT NULL,
    tag TEXT NOT NULL,
    FOREIGN KEY(fk_post_id) REFERENCES posts(id)
);