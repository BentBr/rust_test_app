CREATE TABLE users (
   id SERIAL PRIMARY KEY,
   uuid UUID NOT NULL UNIQUE,
   username VARCHAR NOT NULL,
   salutation VARCHAR NOT NULL DEFAULT '',
   first_name VARCHAR NOT NULL DEFAULT '',
   last_name VARCHAR NOT NULL DEFAULT '',
   email VARCHAR NOT NULL UNIQUE,
   password VARCHAR NOT NULL
);

INSERT INTO users (uuid, username, email, password) VALUES ('b860706a-3739-4f2d-9fe1-aeb2445d50d0', 'placeholder', 'placeholder@email.com', 'placeholder password');

ALTER TABLE to_do ADD user_id integer default 1 CONSTRAINT user_id REFERENCES users NOT NULL;