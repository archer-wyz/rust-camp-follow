-- CREATE TYPE message_type AS enum ('email', 'sms', 'inapp', 'unknown');
--
-- -- Add migration script here
-- CREATE TABLE messages (
--   id VARCHAR(64) PRIMARY KEY,
--   type message_type DEFAULT 'unknown' NOT NULL,
--   sender VARCHAR(64) NOT NULL,
--   body TEXT NOT NULL,
--   created_at TIMESTAMP NOT NULL,
--   updated_at TIMESTAMP NOT NULL,
--   subject VARCHAR(255),
--   recipients VARCHAR(128)[],
--   device_id VARCHAR(64),
--   title VARCHAR(255),
--   times INT NOT NULL
-- );

INSERT INTO messages(id, type, sender, body, created_at, updated_at, subject, recipients, device_id, title, times)
VALUES ('1', 'email', 'sender', 'body', '2024-07-11 14:43:34', '2024-07-11 14:43:34', 'subject', '{recipient1, recipient2}', 'device_id', 'title', 1);

INSERT INTO messages(id, type, sender, body, created_at, updated_at, subject, recipients, device_id, title, times)
VALUES ('2', 'email', 'sender1', 'body1', '2024-07-10 14:43:34', '2024-07-11 14:43:34', 'subject', '{recipient1, recipient2}', 'device_id', 'title', 1);

INSERT INTO messages(id, type, sender, body, created_at, updated_at, subject, recipients, device_id, title, times)
VALUES ('3', 'email', 'sender2', 'body2', '2024-07-9 14:43:34', '2024-07-11 14:43:34', 'subject', '{recipient1, recipient2}', 'device_id', 'title', 1);
