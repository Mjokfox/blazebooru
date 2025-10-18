CREATE FUNCTION update_user(
  IN p_update_user update_user
)
RETURNS boolean
LANGUAGE plpgsql

AS $BODY$
BEGIN
  -- Update user
  UPDATE users
  SET
    name = COALESCE(p_update_user.name, name),
    rank = COALESCE(p_update_user.rank, rank),
    biography = COALESCE(p_update_user.biography, biography),
    css = COALESCE(p_update_user.css, css)
  WHERE id = p_update_user.id;

  RETURN true;
END;
$BODY$;