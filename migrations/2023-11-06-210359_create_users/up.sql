-- Your SQL goes here
CREATE TABLE users (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL,
  email TEXT UNIQUE NOT NULL,
  password TEXT NOT NULL,
  active BOOLEAN NOT NULL DEFAULT FALSE,
  registration_date DATE
)
