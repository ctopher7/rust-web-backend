CREATE TABLE IF NOT EXISTS user_roles(
    id SERIAL PRIMARY KEY NOT NULL,
    role VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER updated_at
BEFORE UPDATE ON user_roles
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

INSERT INTO user_roles(role) VALUES ('superadmin');
INSERT INTO user_roles(role) VALUES ('admin');
INSERT INTO user_roles(role) VALUES ('customer');