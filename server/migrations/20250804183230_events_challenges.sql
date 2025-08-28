CREATE TABLE events_challenges (
  event_uuid UUID NOT NULL REFERENCES events (uuid) ON DELETE CASCADE,
  challenge_uuid UUID NOT NULL REFERENCES challenges (uuid) ON DELETE CASCADE,
  PRIMARY KEY (event_uuid, challenge_uuid)
);
