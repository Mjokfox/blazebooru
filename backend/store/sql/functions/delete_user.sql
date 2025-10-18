CREATE FUNCTION delete_user(
  IN p_user_id integer
)
RETURNS boolean
LANGUAGE plpgsql

AS $BODY$
BEGIN
  -- Delete user
  DELETE FROM users WHERE id = p_user_id;

  -- Invalidate active sessions
  UPDATE refresh_token
  SET used = true
  WHERE user_id = p_user_id;

  RETURN true;
END;
$BODY$;