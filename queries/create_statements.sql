-- name: create_users_table
CREATE TABLE users (
  id INTEGER PRIMARY KEY
);

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

-- name: create_armor_table
CREATE TABLE armor (
  id                  INTEGER PRIMARY KEY,
  user_id             INTEGER NOT NULL,
  equipment_type      INTEGER NOT NULL,
  armor_id            INTEGER NOT NULL,
  defence             INTEGER NOT NULL,
  rarity              INTEGER NOT NULL
);

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
