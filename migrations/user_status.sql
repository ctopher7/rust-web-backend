CREATE TABLE IF NOT EXISTS user_status(
    id SERIAL PRIMARY KEY NOT NULL,
    status VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER updated_at
BEFORE UPDATE ON user_status
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

INSERT INTO user_status(status) VALUES ('unverified');
INSERT INTO user_status(status) VALUES ('verified');