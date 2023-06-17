CREATE TABLE IF NOT EXISTS report_line_items (
    id bigint GENERATED ALWAYS AS IDENTITY,
    report_id bigint NOT NULL,
    item_name VARCHAR(255) NOT NULL,
    item_price_usd money NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_report
        FOREIGN KEY(report_id)
            REFERENCES reports(id)
            ON DELETE CASCADE
);
