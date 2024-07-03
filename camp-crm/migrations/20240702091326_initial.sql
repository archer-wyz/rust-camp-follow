-- Add migration script here
CREATE TABLE IF NOT EXISTS user_stats (
  email VARCHAR(64) NOT NULL PRIMARY KEY,
  name VARCHAR(64) NOT NULL,
  create_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_visited_at TIMESTAMP,
  last_watch_at TIMESTAMP,
  recent_watched int[],
  started_but_not_finished int[],
  finished int[],
  last_email_notification TIMESTAMP,
  last_in_app_notification TIMESTAMP,
  last_sms_notification TIMESTAMP
);
