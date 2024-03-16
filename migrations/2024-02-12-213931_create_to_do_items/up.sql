CREATE TABLE to_do (
    id SERIAL PRIMARY KEY,
    uuid UUID NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    status status NOT NULL DEFAULT 'Open',
    date TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted BOOLEAN NOT NULL DEFAULT false
)