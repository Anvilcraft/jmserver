CREATE TABLE IF NOT EXISTS categories (num INT UNIQUE NOT NULL , id varchar(255) NOT NULL , name TEXT, PRIMARY KEY (id));
CREATE TABLE IF NOT EXISTS users (id varchar(255) NOT NULL, name TEXT, authsource JSON, PRIMARY KEY (id));
CREATE TABLE IF NOT EXISTS memes (id SERIAL, filename varchar(255) NOT NULL, userid varchar(255) NOT NULL, category varchar(255), timestamp TIMESTAMP, ip varchar(255), cid varchar(255) NOT NULL, PRIMARY KEY (id), FOREIGN KEY (category) REFERENCES categories(id), FOREIGN KEY (userid) REFERENCES users(id));
CREATE TABLE IF NOT EXISTS token (uid varchar(255) UNIQUE NOT NULL, token varchar(255), FOREIGN KEY (uid) REFERENCES users(id));
CREATE OR REPLACE FUNCTION UNIX_TIMESTAMP(ts TIMESTAMP) RETURNS INT AS $$
        BEGIN
                RETURN extract(epoch FROM ts)::integer;
        END;
$$ LANGUAGE plpgsql;