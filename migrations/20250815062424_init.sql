-- Add migration script here
CREATE TABLE users (
  id UUID PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  creat_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
