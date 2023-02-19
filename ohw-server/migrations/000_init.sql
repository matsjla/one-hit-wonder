CREATE TABLE IF NOT EXISTS notes
(
    id
    UUID
    PRIMARY
    KEY,
    content
    TEXT
    NOT
    NULL,
    confidential
    BOOLEAN
    NOT
    NULL
);