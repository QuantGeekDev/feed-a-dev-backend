ALTER TABLE snacks
    ADD COLUMN user_id INTEGER;

INSERT INTO users (username, password_hash, role, created_at, updated_at)
SELECT 'admin',
       '$2a$12$K6O5bFK6.O4V6csO94BFN.yvMkv4pOXxYH.rolz4.Y4.pxpwB6fvC',
       'admin',
       CURRENT_TIMESTAMP,
       CURRENT_TIMESTAMP
    WHERE NOT EXISTS (
    SELECT 1 FROM users WHERE username = 'admin'
) RETURNING id;

UPDATE snacks
SET user_id = (SELECT id FROM users WHERE username = 'admin')
WHERE user_id IS NULL;

ALTER TABLE snacks
    ALTER COLUMN user_id SET NOT NULL;

ALTER TABLE snacks
    ADD CONSTRAINT fk_snacks_user
        FOREIGN KEY (user_id)
            REFERENCES users(id)
            ON DELETE CASCADE;

CREATE INDEX idx_snacks_user_id ON snacks(user_id);