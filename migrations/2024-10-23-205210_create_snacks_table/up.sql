CREATE TABLE snacks
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR        NOT NULL,
    category   VARCHAR        NOT NULL,
    price      DECIMAL(10, 2) NOT NULL,
    image_url  VARCHAR        NOT NULL,
    created_at TIMESTAMP      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP      NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('snacks');

CREATE INDEX idx_snacks_category ON snacks (category);
CREATE INDEX idx_snacks_name ON snacks (name);
 
ALTER TABLE snacks
    ADD CONSTRAINT positive_price CHECK (price > 0);
