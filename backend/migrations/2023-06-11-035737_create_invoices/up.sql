CREATE TABLE IF NOT EXISTS invoices (
    invoice_id bigint GENERATED ALWAYS AS IDENTITY,
    owner_id bigint NOT NULL,
    title VARCHAR(255) NOT NULL,
    PRIMARY KEY(invoice_id),
    CONSTRAINT fk_owner
        FOREIGN KEY(owner_id)
            REFERENCES users(user_id)
            ON DELETE CASCADE
);
