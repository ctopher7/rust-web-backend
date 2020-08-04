CREATE TABLE IF NOT EXISTS users(
    id BIGSERIAL PRIMARY KEY NOT NULL,
    user_role_id INT NOT NULL,
    user_status_id INT NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    phone_number VARCHAR(255) NOT NULL UNIQUE,
    password CHAR(60) NOT NULL,
    last_logged_in TIMESTAMPTZ NOT NULL,
    name VARCHAR(255),
    date_of_birth DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();