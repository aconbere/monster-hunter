-- name: insert_messages
INSERT INTO messages (
  source,
  message_type,
  equipment_id,
  idx,
  text
) VALUES (
  $1, $2, $3, $4, $5
);

