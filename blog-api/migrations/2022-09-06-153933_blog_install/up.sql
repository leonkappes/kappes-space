-- Your SQL goes here
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    name varchar(256) NOT NULL,
    password varchar(256) NOT NULL,
    permission varchar(256) NOT NULL
);
CREATE TABLE posts (
    id BIGSERIAL PRIMARY KEY,
    title varchar(256) NOT NULL,
    author BIGSERIAL NOT NULL,
    content TEXT NOT NULL,
    content_md TEXT NOT NULL,
    published varchar(256) NOT NULL,
    FOREIGN KEY (author) REFERENCES users(id)
);