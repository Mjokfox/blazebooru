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