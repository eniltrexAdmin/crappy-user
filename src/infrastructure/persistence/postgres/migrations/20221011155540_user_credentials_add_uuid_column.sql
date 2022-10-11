-- Add migration script here
alter table user_credentials_view
add column user_id uuid NOT NULL