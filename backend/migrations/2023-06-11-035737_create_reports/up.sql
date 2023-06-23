CREATE TABLE IF NOT EXISTS reports (
    id bigint GENERATED ALWAYS AS IDENTITY,
    owner_id bigint NOT NULL,
    title VARCHAR(255) NOT NULL,
    description text,
    PRIMARY KEY(id),
    CONSTRAINT fk_owner
        FOREIGN KEY(owner_id)
            REFERENCES users(id)
            ON DELETE CASCADE
);
