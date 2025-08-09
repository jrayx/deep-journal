PRAGMA foreign_keys = ON;

CREATE TABLE models (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE chats (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL
);

CREATE TABLE messages (
    id INTEGER PRIMARY KEY,
    chat_id INTEGER NOT NULL,
    model_id INTEGER NOT NULL,
    message TEXT NOT NULL,
    FOREIGN KEY (chat_id) REFERENCES chats(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (model_id) REFERENCES models(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);