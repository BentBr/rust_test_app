CREATE TABLE to_do (
    id SERIAL PRIMARY KEY,
    uuid UUID NOT NULL UNIQUE,
    title VARCHAR NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    status status NOT NULL DEFAULT 'Open',
    creation_date TIMESTAMP NOT NULL DEFAULT NOW(),
    modification_date TIMESTAMP,
    deletion_date TIMESTAMP
)