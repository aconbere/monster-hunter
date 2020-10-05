-- name: insert_archive_messages
INSERT INTO archive_messages (
  source,
  source_name,
  message_type,
  equipment_id,
  idx,
  text
) VALUES (
  $1, $2, $3, $4, $5, $6
);

