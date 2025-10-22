CREATE TYPE update_wiki AS (
    id integer,
    title text,
    body text,
    locked boolean,
    deleted boolean,
    reason text
);
