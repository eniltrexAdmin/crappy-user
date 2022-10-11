CREATE TABLE user_read_model_view
(
    uuid  uuid  NOT NULL,
    email text   NOT NULL,
    password_hash text   NOT NULL,
    active bool NOT NULL,
    registered_at timestamptz NOT NULL,
    activated_at timestamptz,
    last_login  timestamptz,
    successful_login_attempts integer,
    unsuccessful_login_attempts integer,
    PRIMARY KEY (uuid)
);