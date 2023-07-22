-- Your SQL goes here
CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	username TEXT UNIQUE NOT NULL,
	email TEXT UNIQUE NOT NULL,
	password TEXT NOT NULL
)