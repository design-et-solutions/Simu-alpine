CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE events (
  uuid UUID PRIMARY KEY DEFAULT gen_random_uuid (),
  owner_uuid UUID NOT NULL REFERENCES users (uuid) ON DELETE CASCADE,
  name TEXT NOT NULL,
  description TEXT,
  status status NOT NULL DEFAULT 'draft',
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
UPDATE ON events FOR EACH ROW EXECUTE PROCEDURE update_updated_at_column ();

-- Indexes
CREATE INDEX idx_events_owner_uuid ON events (owner_uuid);
