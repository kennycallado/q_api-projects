CREATE TABLE IF NOT EXISTS projects ();

ALTER TABLE projects
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN name VARCHAR NOT NULL,
  ADD COLUMN keys TEXT[] NOT NULL DEFAULT '{}';


INSERT INTO projects (name, keys) VALUES
  ('Project 1', '{step, mood}'),
  ('Project 2', default)
  ;

ALTER TABLE projects REPLICA IDENTITY FULL;
CREATE PUBLICATION projects_pub FOR TABLE projects;
