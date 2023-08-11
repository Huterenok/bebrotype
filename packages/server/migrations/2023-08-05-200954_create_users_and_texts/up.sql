-- Your SQL goes here
CREATE TABLE users (
	id SERIAL PRIMARY KEY,

  username TEXT UNIQUE NOT NULL,
  email TEXT UNIQUE NOT NULL,
  password TEXT NOT NULL,
	avatar TEXT,
	
	near_address TEXT UNIQUE,

	favourite_texts INTEGER[] NOT NULL check (array_position(favourite_texts, null) is null) DEFAULT '{}'
);

CREATE TABLE texts (
	id SERIAL PRIMARY KEY,
	title TEXT NOT NULL,
	content TEXT NOT NULL,
	likes INTEGER NOT NULL DEFAULT 0,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE
)