CREATE TABLE spots (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    location POINT NOT NULL
)
