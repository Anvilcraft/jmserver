CREATE TABLE IF NOT EXISTS categories (num INT UNIQUE NOT NULL , id varchar(255) NOT NULL , name TEXT, PRIMARY KEY (id));
CREATE TABLE IF NOT EXISTS users (id varchar(255) NOT NULL, name TEXT, authsource JSON, PRIMARY KEY (id));
CREATE TABLE IF NOT EXISTS memes (id INT NOT NULL AUTO_INCREMENT, filename varchar(255) NOT NULL, user varchar(255) NOT NULL, category varchar(255), timestamp DATETIME, ip varchar(255), PRIMARY KEY (id), FOREIGN KEY (category) REFERENCES categories(id), FOREIGN KEY (user) REFERENCES users(id));
CREATE TABLE IF NOT EXISTS token (uid varchar(255) UNIQUE NOT NULL, token varchar(255), FOREIGN KEY (uid) REFERENCES users(id));
