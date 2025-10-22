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