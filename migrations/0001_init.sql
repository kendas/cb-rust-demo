CREATE TABLE hours (
    id UUID PRIMARY KEY,
    employee VARCHAR NOT NULL,
    date DATE NOT NULL,
    project VARCHAR NOT NULL,
    story_id VARCHAR,
    description VARCHAR NOT NULL,
    hours INT2 NOT NULL
)
