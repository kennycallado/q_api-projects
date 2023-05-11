CREATE TABLE IF NOT EXISTS values ();

ALTER TABLE values
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN user_id INTEGER NOT NULL,
  ADD COLUMN value JSONB NOT NULL;

INSERT INTO values (user_id, value) VALUES
  (1, '{"step": 1, "animo": 50}'),
  (2, '{"step": 1, "animo": 10}'),
  (1, '{"step": 2, "animo": 60}'),
  (1, '{"step": 3, "animo": 65}')
  ;
