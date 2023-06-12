CREATE TABLE IF NOT EXISTS project_records ();

ALTER TABLE project_records
  ADD COLUMN id SERIAL NOT NULL PRIMARY KEY,
  ADD COLUMN project_id INTEGER NOT NULL,
  ADD COLUMN record_id INTEGER NOT NULL;

ALTER TABLE project_records
  ADD CONSTRAINT fk_pv_project_id FOREIGN KEY (project_id) REFERENCES projects (id),
  ADD CONSTRAINT fk_pv_record_id  FOREIGN KEY (record_id) REFERENCES records (id);

INSERT INTO project_records (project_id, record_id) VALUES
  (1, 1)
  ;
