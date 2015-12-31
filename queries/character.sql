--name: insert_talismen
INSERT INTO talismens (
  user_id,
  slots,
  talismen_id,
  skill_1_id,
  skill_1_amount,
  skill_2_id,
  skill_2_amount
) VALUES (
  $1, $2, $3, $4, $5, $6, $7
);

--name: insert_armor
INSERT INTO armor (
  user_id,
  equipment_type,
  armor_id,
  defence,
  rarity
) VALUES (
  $1, $2, $3, $4, $5
);

--name: insert_weapon
INSERT INTO weapons (
  user_id,
  weapon_id,
  equipment_type,
  element_value,
  element_type,
  sharpness,
  rarity,
  hone_type
) VALUES (
  $1, $2, $3, $4, $5, $6, $7, $8
);
