CREATE TABLE IF NOT EXISTS report_permissions (
    id bigint GENERATED ALWAYS AS IDENTITY,
    borrower_id bigint NOT NULL,
    report_id bigint NOT NULL,
    read_access boolean NOT NULL,
    write_access boolean NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_borrower
        FOREIGN KEY(borrower_id)
            REFERENCES users(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_report
        FOREIGN KEY(report_id)
            REFERENCES reports(id)
            ON DELETE CASCADE
);
