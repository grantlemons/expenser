CREATE TABLE IF NOT EXISTS invoice_line_items (
    id bigint GENERATED ALWAYS AS IDENTITY,
    invoice_id bigint NOT NULL,
    item_name VARCHAR(255) NOT NULL,
    item_price_usd money NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_invoice
        FOREIGN KEY(invoice_id)
            REFERENCES invoices(invoice_id)
            ON DELETE CASCADE
);
