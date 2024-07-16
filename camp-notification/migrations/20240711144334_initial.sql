CREATE TYPE message_type AS enum ('email', 'sms', 'inapp', 'unknown');

-- Add migration script here
CREATE TABLE messages (
  id VARCHAR(64) NOT NULL,
  type message_type DEFAULT 'unknown' NOT NULL,
  sender VARCHAR(64) NOT NULL,
  body TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  subject VARCHAR(255),
  recipients VARCHAR(128)[],
  device_id VARCHAR(64),
  title VARCHAR(255),
  times INT NOT NULL
);
