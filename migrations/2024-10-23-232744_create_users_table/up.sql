CREATE TABLE users
(
    id            SERIAL PRIMARY KEY,
    username      VARCHAR   NOT NULL UNIQUE,
    password_hash VARCHAR   NOT NULL,
    role          VARCHAR   NOT NULL DEFAULT 'user',
    created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('users');

CREATE INDEX idx_users_username ON users (username);

ALTER TABLE users
    ADD CONSTRAINT valid_role CHECK (role IN ('user', 'admin'));
