use rusqlite::Connection;
use rawsql::Loader;

use objects::character::Character;
use objects::equipment::DecodedEquipmentClass;
use objects::archive::{MessageCollection};

static CHARACTER_TABLES: &'static [&'static str] = &[
    "users",
    "game_data",
    "user_features",
    "talismens",
    "armor",
    "weapons",
];

static ARCHIVE_TABLES: &'static [&'static str] = &[
    "messages",
];

fn drop_tables(conn: &Connection, tables:&[&str]) {
    let queries = Loader::get_queries_from("queries/create_statements.sql").unwrap().queries;

    for table in tables.iter() {
        let statement_name = format!("drop_{}_table", table);
        let create = queries.get(&statement_name).unwrap();
        conn.execute(&create, &[]).unwrap();
    }
}

fn create_tables(conn: &Connection, tables:&[&str]) {
    let queries = Loader::get_queries_from("queries/create_statements.sql").unwrap().queries;

    for table in tables.iter() {
        let statement_name = format!("create_{}_table", table);
        let create = queries.get(&statement_name).unwrap();
        conn.execute(&create, &[]).unwrap();
    }
}

fn insert_character(conn: &Connection, character:&Character) {
    let queries = Loader::get_queries_from("queries/character.sql").unwrap().queries;
    let insert_talismen = queries.get("insert_talismen").unwrap();
    let insert_armor = queries.get("insert_armor").unwrap();
    let insert_weapon = queries.get("insert_weapon").unwrap();

    for equipment in character.equipment_box.iter() {
        match equipment.decode() {
            DecodedEquipmentClass::Talismen(talismen) => {
                conn.execute(insert_talismen, &[
                    &1,
                    &(talismen.number_of_slots as i64),
                    &(talismen.talismen_id as i64),
                    &(talismen.skill_1_id as i64),
                    &(talismen.skill_1_amount as i64),
                    &(talismen.skill_2_id as i64),
                    &(talismen.skill_2_amount as i64)
                ]).unwrap();
            },
            DecodedEquipmentClass::Armor(armor) => {
                conn.execute(insert_armor, &[
                    &1,
                    &(armor.equipment_type as i64),
                    &(armor.armor_id as i64),
                    &(armor.defence as i64),
                    &(armor.rarity as i64),
                ]).unwrap();
            },
            DecodedEquipmentClass::Weapon(weapon) => {
                conn.execute(insert_weapon, &[
                    &1,
                    &(weapon.weapon_id as i64),
                    &(weapon.equipment_type as i64),
                    &(weapon.element_value as i64),
                    &(weapon.element_type as i64),
                    &(weapon.sharpness as i64),
                    &(weapon.rarity as i64),
                    &(weapon.hone_type as i64)
                ]).unwrap();
            },
            _ => {
            }
        }
    }
}

pub fn export_save(destination: &str, character: &Character) {
    let conn = Connection::open(destination).unwrap();
    drop_tables(&conn, CHARACTER_TABLES);
    create_tables(&conn, CHARACTER_TABLES);
    insert_character(&conn, character)
}

pub fn export_archive(destination: &str, message_collections:Vec<MessageCollection>) {
    let conn = Connection::open(destination).unwrap();
    let queries = Loader::get_queries_from("queries/archive.sql").unwrap().queries;
    let insert_messages = queries.get("insert_messages").unwrap();

    drop_tables(&conn, ARCHIVE_TABLES);
    create_tables(&conn, ARCHIVE_TABLES);

    for col in message_collections {
        for (i, m) in col.messages.iter().enumerate() {
            conn.execute(insert_messages, &[
                &col.source,
                &col.source_name,
                &(i as i64),
                m
            ]).unwrap();
        }
    }
}
