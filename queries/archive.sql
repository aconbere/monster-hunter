-- name: insert_messages
INSERT INTO messages (
  source,
  type,
  idx,
  text
) VALUES (
  $1, $2, $3, $4
);

