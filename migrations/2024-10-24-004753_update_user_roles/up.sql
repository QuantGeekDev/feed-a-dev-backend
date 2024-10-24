ALTER TABLE users DROP CONSTRAINT valid_role;
ALTER TABLE users ADD CONSTRAINT valid_role
    CHECK (role IN ('admin', 'project_manager', 'developer'));

UPDATE users SET role = 'developer' WHERE role = 'user';
