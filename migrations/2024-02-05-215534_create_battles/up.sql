-- Your SQL goes here
CREATE TABLE battles (
    id varchar PRIMARY KEY,
    monster_a varchar NOT NULL,
    monster_b varchar NOT NULL,
    winner varchar NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    FOREIGN KEY (winner) REFERENCES monsters(id) ON DELETE CASCADE
);
