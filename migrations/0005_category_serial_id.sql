ALTER TABLE task DROP CONSTRAINT fk_category,
    ADD COLUMN category_id integer;
ALTER TABLE category DROP CONSTRAINT category_pkey,
    ADD UNIQUE (name),
    ADD COLUMN id SERIAL PRIMARY KEY;
UPDATE task
SET category_id = (
        SELECT id
        FROM category
        WHERE name = task.category_name
    );
ALTER TABLE task
ALTER COLUMN category_id
SET NOT NULL,
    ADD CONSTRAINT fk_category FOREIGN KEY (category_id) REFERENCES category (id);