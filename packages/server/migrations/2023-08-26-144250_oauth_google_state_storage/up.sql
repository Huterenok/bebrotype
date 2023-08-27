-- Your SQL goes here
CREATE TABLE oauth_google_storage (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	csrf_state TEXT NOT NULL,
	pkce_code_verifier TEXT NOT NULL,
	return_url TEXT NOT NULL
)