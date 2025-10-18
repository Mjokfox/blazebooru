-- Add migration script here

CREATE TYPE update_user AS (
  id integer,

  name text,
  rank smallint,
  biography text,
  css text
);


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



-- fix typo

DROP FUNCTION create_post_comment;

CREATE FUNCTION create_post_comment(
  IN p_comment new_post_comment,
  IN p_user_id integer
)
RETURNS post_comment
LANGUAGE plpgsql

AS $BODY$
DECLARE
  v_comment post_comment;
BEGIN
  -- Insert post
  INSERT INTO post_comment (
    user_id,
    user_name,
    post_id,
    comment
  )
  SELECT
    p_user_id, -- user_id
    (SELECT name FROM users WHERE id = p_user_id), -- user_name
    p_comment.post_id, -- post_id
    p_comment.comment -- comment
  RETURNING * INTO v_comment;

  RETURN v_comment;
END;
$BODY$;