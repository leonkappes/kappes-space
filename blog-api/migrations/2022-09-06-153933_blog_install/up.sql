-- Your SQL goes here
CREATE TABLE posts (
    id BIGSERIAL PRIMARY KEY,
    title varchar(256) NOT NULL,
    author varchar(256) NOT NULL,
    published varchar(256) NOT NULL
);