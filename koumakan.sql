PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE comment(id int primary key, article_id int, name varchar(255), email varchar(255), website varchar(255), content varchar(255), avatar varchar(255), date datetime);
COMMIT;
