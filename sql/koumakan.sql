PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE article(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(255),
    brief VARCHAR(1024),
    content TEXT,
    typestring VARCHAR(255),
    labels VARCHAR(255),
    date DATETIME);
CREATE TABLE comment(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    article_id INTEGER,
    name VARCHAR(255),
    email VARCHAR(255),
    website VARCHAR(255),
    content VARCHAR(255),
    avatar VARCHAR(255),
    date DATETIME);
COMMIT;
