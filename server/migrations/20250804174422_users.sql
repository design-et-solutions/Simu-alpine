CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE users (
  uuid UUID PRIMARY KEY DEFAULT gen_random_uuid (),
  name TEXT NOT NULL,
  lang char(2) NOT NULL REFERENCES country (code),
  deleted BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Trigger function for auto-updating `updated_at`
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = NOW();
   RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply the trigger
CREATE TRIGGER set_updated_at BEFORE
UPDATE ON users FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column ();
