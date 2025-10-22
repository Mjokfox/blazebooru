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
