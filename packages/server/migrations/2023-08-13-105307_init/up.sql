-- Your SQL goes here
CREATE TABLE users (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  username TEXT UNIQUE NOT NULL,
  email TEXT UNIQUE NOT NULL,
  password TEXT NOT NULL,
	avatar TEXT,
	near_address TEXT UNIQUE
);

CREATE TABLE texts (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	title TEXT NOT NULL,
	content TEXT NOT NULL,
	likes INTEGER NOT NULL DEFAULT 0,
	user_id BIGINT NOT NULL,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE liked_texts (
  user_id BIGINT,
  text_id BIGINT,

	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
	FOREIGN KEY (text_id) REFERENCES texts(id) ON DELETE CASCADE,

  PRIMARY KEY (user_id, text_id)
);