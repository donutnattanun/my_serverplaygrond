-- Add migration script here
-- make sure extension exists
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- add default to id column
ALTER TABLE users
ALTER COLUMN id SET DEFAULT uuid_generate_v4();

