---- TABLES

CREATE TABLE wiki_pages (
    id serial NOT NULL,
    creator_id integer NOT NULL,
    title text NOT NULL,
    body text NOT NULL,
    locked boolean DEFAULT false NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    revision integer DEFAULT 1 NOT NULL,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater_id integer NOT NULL,
    updater_ip inet NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    reason text,

    PRIMARY KEY (id),
    UNIQUE(title),
    FOREIGN KEY (creator_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
        NOT VALID,
    FOREIGN KEY (updater_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
        NOT VALID
);

SELECT manage_updated_at('wiki_pages'); -- Automatically manage updated_at

CREATE TABLE wiki_pages_history (
    id serial NOT NULL,
    wiki_page_id integer NOT NULL,
    updater_id integer NOT NULL,
    updater_ip inet NOT NULL,
    title text NOT NULL,
    body text NOT NULL,
    locked boolean NOT NULL,
    revision integer DEFAULT 1 NOT NULL,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted boolean DEFAULT false NOT NULL,
    reason text,

    PRIMARY KEY (id),
    FOREIGN KEY (wiki_page_id)
        REFERENCES wiki_pages (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
        NOT VALID,
    FOREIGN KEY (updater_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
        NOT VALID
);

---- TYPES

CREATE TYPE new_wiki AS (
    title text,
    body text,
    locked boolean,
    reason text
);

CREATE TYPE update_wiki AS (
    id integer,
    title text,
    body text,
    locked boolean,
    deleted boolean,
    reason text
);


---- VIEWS

CREATE VIEW view_wiki
AS
SELECT
  w.id,
  w.creator_id,
  ud.name AS creator_name,
  w.title,
  w.body,
  w.locked,
  w.created_at,
  w.revision,
  w.updated_at,
  w.updater_id,
  up.name AS updater_name,
  w.updater_ip,
  w.deleted,
  w.reason
FROM wiki_pages AS w
JOIN users AS ud ON ud.id = w.creator_id
JOIN users AS up ON up.id = w.updater_id;

---- FUNCTIONS

-- create

CREATE FUNCTION create_wiki(
  IN p_wiki new_wiki,
  IN user_id integer,
  IN user_ip inet 
)
RETURNS SETOF view_wiki
LANGUAGE plpgsql

AS $BODY$
DECLARE
  v_wiki_id integer;
BEGIN
  -- Insert post
  INSERT INTO wiki_pages (
    creator_id,
    title,
    body,
    locked,
    updater_id,
	updater_ip,
    reason
  )
  SELECT
    user_id, -- creator_id
    p_wiki.title, -- title
    p_wiki.body, -- body
    p_wiki.locked, -- locked
    user_id, -- updater_id
	  user_ip, -- updater_ip
    p_wiki.reason -- reason
    RETURNING id INTO v_wiki_id;

  -- return the view
  RETURN QUERY
  SELECT vw.*
  FROM view_wiki vw
  WHERE vw.id = v_wiki_id;
END;
$BODY$;

-- update

CREATE FUNCTION update_wiki(
  IN p_wiki update_wiki,
  IN user_id integer,
  IN user_ip inet 
)
RETURNS boolean
LANGUAGE plpgsql
AS $BODY$
DECLARE
  o_wiki wiki_pages;
BEGIN
  -- Fetch the old wiki page
  SELECT * INTO o_wiki
  FROM wiki_pages
  WHERE id = p_wiki.id;

  -- If not found, exit early
  IF NOT FOUND THEN
    RAISE NOTICE 'Wiki page with id % not found', p_wiki.id;
    RETURN false;
  END IF;

  -- Insert the old page into history
  INSERT INTO wiki_pages_history (
    wiki_page_id,
    updater_id,
    updater_ip,
    title,
    body,
    locked,
    revision,
    updated_at,
    deleted,
    reason
  ) SELECT 
    o_wiki.id, -- wiki_page_id
    o_wiki.updater_id, -- updater_id
    o_wiki.updater_ip, -- updater_ip
    o_wiki.title, -- title
    o_wiki.body, -- body
    o_wiki.locked, -- locked
    o_wiki.revision, -- revision
    o_wiki.updated_at, -- updated_at
    o_wiki.deleted, -- deleted
    o_wiki.reason; -- reason

  -- Insert updated page
  UPDATE wiki_pages
  set 
    title = p_wiki.title,
    body = p_wiki.body,
    revision = o_wiki.revision + 1,
    locked = p_wiki.locked,
    deleted = p_wiki.deleted,
    reason = p_wiki.reason,
    updater_id = user_id,
    updater_ip = user_ip
  WHERE id = p_wiki.id;

  RETURN true;
END;
$BODY$;

-- delete 

CREATE FUNCTION delete_wiki(
  IN p_wiki_name text
)
RETURNS boolean
LANGUAGE plpgsql

AS $BODY$
BEGIN
  -- Update deleted value
  UPDATE wiki_pages SET deleted = true WHERE title = p_wiki_name;

  RETURN true;
END;
$BODY$;