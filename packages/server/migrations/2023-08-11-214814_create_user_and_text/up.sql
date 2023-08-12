-- Your SQL goes here
CREATE TABLE users (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

  username TEXT UNIQUE NOT NULL,
  email TEXT UNIQUE NOT NULL,
  password TEXT NOT NULL,
	avatar TEXT,
	
	near_address TEXT UNIQUE,

	favourite_texts BIGINT[] NOT NULL check (array_position(favourite_texts, null) is null) DEFAULT '{}'
);

CREATE TABLE texts (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	title TEXT NOT NULL,
	content TEXT NOT NULL,
	likes INTEGER NOT NULL DEFAULT 0,
	user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE
)