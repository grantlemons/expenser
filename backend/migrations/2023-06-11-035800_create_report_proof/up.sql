CREATE TABLE IF NOT EXISTS report_proof (
    id bigint GENERATED ALWAYS AS IDENTITY,
    report_id bigint NOT NULL,
    data BYTEA NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_report
        FOREIGN KEY(report_id)
            REFERENCES reports(id)
            ON DELETE CASCADE
);
