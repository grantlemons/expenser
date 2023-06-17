CREATE TABLE IF NOT EXISTS users (
    id bigint GENERATED ALWAYS AS IDENTITY,
    username VARCHAR(128) NOT NULL,
    email VARCHAR(128) UNIQUE NOT NULL,
    profile_picture bytea,
    password_hash VARCHAR(255) NOT NULL,
    PRIMARY KEY(id)
);
