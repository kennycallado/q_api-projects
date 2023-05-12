CREATE TABLE IF NOT EXISTS records ();

ALTER TABLE records
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN user_id INTEGER NOT NULL,
  ADD COLUMN record JSONB NOT NULL;

INSERT INTO records (user_id, record) VALUES
  (1, '{"step": 1, "animo": 50}'),
  (2, '{"step": 1, "animo": 10}'),
  (1, '{"step": 2, "animo": 60}'),
  (1, '{"step": 3, "animo": 65}')
  ;
