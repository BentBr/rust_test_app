ALTER TABLE users
    ADD creation_date TIMESTAMP NOT NULL DEFAULT NOW(),
    ADD modification_date TIMESTAMP,
    ADD deletion_date TIMESTAMP;