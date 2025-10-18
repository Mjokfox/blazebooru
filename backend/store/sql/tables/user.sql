CREATE TABLE users
(
  id serial NOT NULL,
  created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
  name text NOT NULL,
  password_hash text NOT NULL,
  rank smallint NOT NULL DEFAULT 0,
  biography text,
  css text,

  PRIMARY KEY (id),
  UNIQUE (name)
);

SELECT manage_updated_at('users'); -- Automatically manage updated_at
