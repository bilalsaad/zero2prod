-- Add migration script here
DROP TABLE idemoptency;

CREATE TABLE idempotency (
  user_id uuid NOT NULL REFERENCES users(user_id),
  idempotency_key TEXT NOT NULL,
  response_status_code SMALLINT NOT NULL,
  response_headers header_pair[] NOT NULL,
  response_body BYTEA,
  created_at timestamptz NOT NULL,
  PRIMARY KEY(user_id, idempotency_key)
);
