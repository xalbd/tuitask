CREATE TABLE category (name varchar PRIMARY KEY);
INSERT INTO category (name)
VALUES ('Default Category');
ALTER TABLE task
ADD COLUMN category_name varchar;
UPDATE task
SET category_name = 'Default Category';
ALTER TABLE task
ADD CONSTRAINT fk_category FOREIGN KEY (category_name) REFERENCES category;