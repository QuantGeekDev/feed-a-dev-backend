ALTER TABLE users DROP CONSTRAINT valid_role;
ALTER TABLE users ADD CONSTRAINT valid_role
    CHECK (role IN ('user', 'admin'));

UPDATE users SET role = 'user' WHERE role = 'developer';