CREATE TABLE IF NOT EXISTS project_values ();

ALTER TABLE project_values
  ADD COLUMN id SERIAL NOT NULL PRIMARY KEY,
  ADD COLUMN project_id INTEGER NOT NULL,
  ADD COLUMN values_id INTEGER NOT NULL;

ALTER TABLE project_values
  ADD CONSTRAINT fk_pv_project_id FOREIGN KEY (project_id) REFERENCES projects (id),
  ADD CONSTRAINT fk_pv_values_id  FOREIGN KEY (values_id) REFERENCES values (id);

INSERT INTO project_values (project_id, values_id) VALUES
  (1, 1),
  (1, 2),
  (1, 3),
  (1, 4)
  ;
