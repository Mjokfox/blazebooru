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