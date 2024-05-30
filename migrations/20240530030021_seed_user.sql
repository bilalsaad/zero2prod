-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
  'ddf8994f-d522-4659-8d02-c1d479057be6',
  'admin',
  -- generated from "everythinghastostartsomewhere" and argon2::default().hash on it.
  '$argon2id$v=19$m=19456,t=2,p=1$rolL5Uv0N3+Zyy7Kk9L/7g$k2+1JSTYMYS93vwa79x0lHtTkCxo8uM1Wg7dMY1RIWU'
);

