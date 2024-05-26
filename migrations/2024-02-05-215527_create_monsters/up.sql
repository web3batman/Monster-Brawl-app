-- Your SQL goes here
CREATE TABLE monsters (
    id varchar PRIMARY KEY,
    name varchar NOT NULL,
    image_url varchar NOT NULL,
    attack INT NOT NULL,
    defense INT NOT NULL,
    hp INT NOT NULL,
    speed INT NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);
