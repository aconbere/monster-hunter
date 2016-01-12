-- name: insert_messages
INSERT INTO messages (
  source,
  type,
  text
) VALUES (
  $1, $2, $3
);

