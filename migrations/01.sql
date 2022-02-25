-- CREATE TYPE inventory_item AS (
--     name text,
--     supplier_id integer
-- );

CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');

CREATE TABLE IF NOT EXISTS rust_test (
    id INT,
    some_bool BOOL,
    name VARCHAR,
    --item inventory_item
    current_mood mood
);