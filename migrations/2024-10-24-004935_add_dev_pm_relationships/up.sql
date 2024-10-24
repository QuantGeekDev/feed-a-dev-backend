CREATE TABLE dev_pm_relationships (
                                      id SERIAL PRIMARY KEY,
                                      developer_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                                      project_manager_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                                      status VARCHAR NOT NULL CHECK (status IN ('pending', 'accepted', 'rejected')),
                                      created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                      updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_dev_pm_developer ON dev_pm_relationships(developer_id);
CREATE INDEX idx_dev_pm_manager ON dev_pm_relationships(project_manager_id);
CREATE INDEX idx_dev_pm_status ON dev_pm_relationships(status);

CREATE UNIQUE INDEX idx_unique_dev_pm_relationship
    ON dev_pm_relationships(developer_id, project_manager_id)
    WHERE status != 'rejected';

SELECT diesel_manage_updated_at('dev_pm_relationships');