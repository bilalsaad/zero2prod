CREATE TYPE header_pair AS (
  name TEXT,
  value BYTEA
);


CREATE TABLE idemoptency (
  user_id uuid NOT NULL REFERENCES users(user_id),
  idemoptency_key TEXT NOT NULL,
  response_status_code SMALLINT NOT NULL,
  response_headers header_pair[] NOT NULL,
  response_body BYTEA,
  created_at timestamptz NOT NULL,
  PRIMARY KEY(user_id, idemoptency_key)
);
