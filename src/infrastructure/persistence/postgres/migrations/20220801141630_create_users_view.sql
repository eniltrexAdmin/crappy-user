CREATE TABLE user_credentials_view
(
    user_email text                        NOT NULL,
    user_hash text                       NOT NULL,
    PRIMARY KEY (user_email)
);
