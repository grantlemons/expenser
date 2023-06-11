CREATE TABLE IF NOT EXISTS invoice_proof (
    proof_id bigint GENERATED ALWAYS AS IDENTITY,
    invoice_id bigint NOT NULL,
    data BYTEA NOT NULL,
    PRIMARY KEY(proof_id),
    CONSTRAINT fk_invoice
        FOREIGN KEY(invoice_id)
            REFERENCES invoices(invoice_id)
            ON DELETE CASCADE
);
