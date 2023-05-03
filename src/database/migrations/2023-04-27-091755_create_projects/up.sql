CREATE TABLE IF NOT EXISTS projects ();
CREATE PUBLICATION projects_pub FOR TABLE projects;

ALTER TABLE projects
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN name VARCHAR NOT NULL,
  ADD COLUMN keys TEXT[] NOT NULL DEFAULT '{}';

ALTER TABLE projects REPLICA IDENTITY FULL;

INSERT INTO projects (name, keys) VALUES
  ('Project 1', '{strength, courage, mood}'),
  ('Project 2', default)
  ;
