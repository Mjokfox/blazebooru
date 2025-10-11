-- Drop old functions
DROP FUNCTION create_user;
DROP FUNCTION can_user_edit_post;
DROP FUNCTION can_user_edit_tag;
DROP FUNCTION create_post_comment;

---- TABLES ----

-- Rename table "user" to users
ALTER TABLE "user"
RENAME TO users; 

-- Add biography column to users
ALTER TABLE users
  ADD COLUMN biography text;

-- Add css column to users
ALTER TABLE users
  ADD COLUMN css text;

---- FUNCTIONS ----

-- edit "user" to users in create_user()
CREATE FUNCTION create_user(
  IN p_user new_user
)
RETURNS users
LANGUAGE plpgsql

AS $BODY$
DECLARE
  v_user users;
  v_rank smallint;
BEGIN
  -- If there are no existing users,
  -- make the new one a giga-admin.
  IF NOT EXISTS(SELECT * FROM users) THEN
    v_rank := 9001;
  ELSE
    v_rank := 0;
  END IF;

  -- Insert user
  INSERT INTO users (
    name,
    password_hash,
    rank
  )
  SELECT
    p_user.name, -- name
    p_user.password_hash, -- password_hash
    v_rank
  RETURNING * INTO v_user;

  RETURN v_user;
END;
$BODY$;

-- edit "user" to users in can_user_edit_post()
CREATE FUNCTION can_user_edit_post(
  IN p_post_id integer,
  IN p_user_id integer
)
RETURNS boolean
LANGUAGE plpgsql

AS $BODY$
BEGIN
  -- If user is some sort of admin, allow edit
  IF (SELECT rank FROM users WHERE id = p_user_id) > 0 THEN
    RETURN true;
  END IF;

  -- If user is the uploader of the post, allow edit
  IF p_user_id = (SELECT user_id FROM post WHERE id = p_post_id) THEN
    RETURN true;
  END IF;

  RETURN false;
END;
$BODY$;

-- edit "user" to users in can_user_edit_tag()
CREATE FUNCTION can_user_edit_tag(
  IN p_tag_id integer,
  IN p_user_id integer
)
RETURNS boolean
LANGUAGE plpgsql

AS $BODY$
BEGIN
  -- If user is some sort of admin, allow edit
  IF (SELECT rank FROM users WHERE id = p_user_id) > 0 THEN
    RETURN true;
  END IF;

  RETURN false;
END;
$BODY$;

-- edit "user" to users in create_post_comment()
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
    (SELECT name FROM susers WHERE id = p_user_id), -- user_name
    p_comment.post_id, -- post_id
    p_comment.comment -- comment
  RETURNING * INTO v_comment;

  RETURN v_comment;
END;
$BODY$;

---- VIEWS ----

-- edit "user" to users in view_post
CREATE OR REPLACE VIEW view_post
AS
SELECT
  p.id,
  p.created_at,
  p.user_id,
  u.name AS user_name,
  p.title,
  p.description,
  p.source,
  p.filename,
  p.size,
  p.width,
  p.height,
  p.hash,
  p.ext,
  p.tn_ext,
  p.tags
FROM post AS p
JOIN users AS u ON u.id = p.user_id
WHERE NOT is_deleted;
