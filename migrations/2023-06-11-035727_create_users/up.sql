CREATE EXTENSION IF NOT EXISTS citext;
CREATE DOMAIN email AS citext
  CHECK ( value ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

CREATE TABLE IF NOT EXISTS users (
    user_id bigint GENERATED ALWAYS AS IDENTITY,
    username VARCHAR(128) NOT NULL,
    email email UNIQUE NOT NULL,
    profile_picture bytea,
    password_hash VARCHAR(128) NOT NULL,
    PRIMARY KEY(user_id)
);
