-- Your SQL goes here
CREATE TABLE IF NOT EXISTS  tabs (
  id INTEGER NOT NULL PRIMARY KEY,
  order_id INTEGER NOT NULL DEFAULT 0,
  is_active BOOLEAN NOT NULL DEFAULT 0,
  create_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  update_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  requests_id INTEGER NOT NULL REFERENCES requests(id)
);