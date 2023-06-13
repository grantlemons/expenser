CREATE TABLE IF NOT EXISTS invoice_permissions (
    access_id bigint GENERATED ALWAYS AS IDENTITY,
    borrower_id bigint NOT NULL,
    invoice_id bigint NOT NULL,
    read_access boolean NOT NULL,
    write_access boolean NOT NULL,
    PRIMARY KEY(access_id),
    CONSTRAINT fk_borrower
        FOREIGN KEY(borrower_id)
            REFERENCES users(user_id)
            ON DELETE CASCADE,
    CONSTRAINT fk_invoice
        FOREIGN KEY(invoice_id)
            REFERENCES invoices(invoice_id)
            ON DELETE CASCADE
);
