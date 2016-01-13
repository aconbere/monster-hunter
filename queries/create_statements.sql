-- name: drop_users_table
DROP TABLE IF EXISTS users;

-- name: create_users_table
CREATE TABLE users (
  id INTEGER PRIMARY KEY
);

-- name: drop_game_data_table
DROP TABLE IF EXISTS game_data;

-- name: create_game_data_table
CREATE TABLE game_data (
  id              INTEGER PRIMARY KEY,
  user_id         INTEGER,
  total_hr_points INTEGER NOT NULL,
  funds           INTEGER NOT NULL,
  play_time_mh4g  INTEGER NOT NULL,
  play_time_mh4   INTEGER NOT NULL
);

-- features_color: Color,
-- hair_color:     Color,
-- clothing_color: Color,
-- skin_tone:      Color,

-- name drop_user_features_table
DROP TABLE IF EXISTS user_features;

-- name create_user_features_table
CREATE TABLE user_features (
  id            INTEGER PRIMARY KEY,
  user_id       INTEGER,
  name          TEXT NOT NULL,
  gender        INTEGER NOT NULL,
  face          INTEGER NOT NULL,
  hair_style    INTEGER NOT NULL,
  clothing_type INTEGER NOT NULL,
  voice         INTEGER NOT NULL,
  eye_color     INTEGER NOT NULL,
  features_type INTEGER NOT NULL
);

-- name: drop_talismens_table
DROP TABLE IF EXISTS talismens;

-- name: create_talismens_table
CREATE TABLE talismens (
  id             INTEGER PRIMARY KEY,
  user_id        INTEGER NOT NULL,
  slots          INTEGER NOT NULL,
  talismen_id    INTEGER NOT NULL,
  skill_1_id     INTEGER NOT NULL,
  skill_1_amount INTEGER NOT NULL,
  skill_2_id     INTEGER NOT NULL,
  skill_2_amount INTEGER NOT NULL
);

-- name: drop_armor_table
DROP TABLE IF EXISTS armor;

-- name: create_armor_table
CREATE TABLE armor (
  id                  INTEGER PRIMARY KEY,
  user_id             INTEGER NOT NULL,
  equipment_type      INTEGER NOT NULL,
  armor_id            INTEGER NOT NULL,
  defence             INTEGER NOT NULL,
  rarity              INTEGER NOT NULL
);

-- name: drop_weapons_table
DROP TABLE IF EXISTS weapons;

-- name: create_weapons_table
CREATE TABLE weapons (
  id             INTEGER PRIMARY KEY,
  user_id        INTEGER NOT NULL,
  equipment_type INTEGER NOT NULL,
  weapon_id      INTEGER NOT NULL,
  element_value  INTEGER NOT NULL,
  element_type   INTEGER NOT NULL,
  sharpness      INTEGER NOT NULL,
  rarity         INTEGER NOT NULL,
  hone_type      INTEGER NOT NULL
);

-- name: drop_messages_table
DROP TABLE IF EXISTS messages;

-- name: create_messages_table
CREATE TABLE messages (
  source       TEXT NOT NULL,
  message_type TEXT NOT NULL,
  equipment_id INTEGER NOT NULL,
  idx          INTEGER NOT NULL,
  text         TEXT NOT NULL
);
