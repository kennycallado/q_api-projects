CREATE TABLE IF NOT EXISTS records ();

ALTER TABLE records
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN user_id INTEGER NOT NULL,
  ADD COLUMN record JSONB NOT NULL;

INSERT INTO records (user_id, record) VALUES
  (1, '{"step": 0, "mood": 0}')
  ;
