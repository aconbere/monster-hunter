use std::fmt;
use std::str;

use objects::weapon::Weapon;
use objects::armor::Armor;
use objects::talismen::Talismen;
use objects::color::Color;
use objects::guild_quest::GuildQuest;
use objects::item::Item;
use objects::palico::Palico;
use objects::palico_equipment::PalicoEquipment;
use objects::equipment::Equipment;
use objects::expedition::Expedition;

#[derive(Debug)]
#[repr(C, packed)]
pub struct PalicoForteCounts {
    pub leadership: u16,
    pub fighting: u16,
    pub protection: u16,
    pub support: u16,
    pub healing: u16,
    pub bombing: u16,
    pub stealing: u16,
    pub treasure: u16,
    pub launching: u16,
    pub stamina: u16,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct EquippedArmor {
    pub chest: Armor,
    pub arms:  Armor,
    pub waist: Armor,
    pub legs:  Armor,
    pub head:  Armor,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct EquipmentBoxIndexes {
    pub weapon:      u16,
    pub chest_armor: u16,
    pub arms_armor:  u16,
    pub waist_armor: u16,
    pub legs_armor:  u16,
    pub head_armor:  u16,
    pub talismen:    u16,
}

#[repr(C, packed)]
pub struct Character {
    pub name:                   [u8; 24],
    pub gender:                 u8,
    pub face:                   u8,
    pub hair_style:             u8,
    pub clothing_type:          u8,
    pub voice:                  u8,
    pub eye_color:              u8,
    pub features_type:          u8,

    pub features_color:         Color,
    pub hair_color:             Color,
    pub clothing_color:         Color,
    pub skin_tone:              Color,

    pub unknown_1:              u8,

    pub hunter_rank:            u32,
    pub total_hr_points:        u32,
    pub funds:                  u32,
    pub play_time_mh4g:         u32,
    pub play_time_mh4:          u32,

    pub equipped_weapon:        Weapon,
    pub equipped_armor:         EquippedArmor,
    pub equipped_talismen:      Talismen,

    pub equipment_box_indexes: EquipmentBoxIndexes,

    pub unknown_2: [u8; 78],

    pub item_box: [Item; 1400],
    pub equipment_box: [Equipment; 1500],
    pub palico_equipment_box: [PalicoEquipment; 600],

    pub unknown_4: u16,

    pub main_palico: Palico,

    pub unknown_5: [u8; 1372],

    pub gunner_pouch: [Item; 8],
    pub item_pouch: [Item; 24],

    pub unknown_6: [u8; 4404],

    pub guild_quests: [GuildQuest; 10],

    pub unknown_7: [u8; 24],

    pub caravan_points: u32,

    pub unknown_8: [u8; 12],

    pub expedition_low: Expedition,
    pub expedition_high: Expedition,
    pub expedition_g: Expedition,

    pub unknown_9: [u8; 1396],

    pub palico_forte_counts: PalicoForteCounts,

    pub unknown_10: [u8; 488],

    pub palico_first_stringers: [Palico; 5],
    pub palico_reserves: [Palico; 50],

    pub unknown_11: [u8; 240],
}

impl Character {
    fn name(&self) -> &str {
        str::from_utf8(&self.name).unwrap()
    }
}

impl fmt::Debug for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,  r#"Character: (
            name: {},
            gender: {},
            face: {},
            hair_style: {},
            clothing_type: {},
            voice: {},
            eye_color: {},
            features_type: {},

            features_color: {:#?},
            hair_color: {:#?},
            clothing_color: {:#?},
            skin_tone: {:#?},

            hunter_rank: {},
            total_hr_points: {},
            funds: {},
            play_time_mh4g: {},
            play_time_mh4: {},
            equipped_weapon: {:#?},
            equipped_armor: {:#?},
            equipped_talismen: {:#?},
        )"#,
            self.name(),
            self.gender,
            self.face,
            self.hair_style,
            self.clothing_type,
            self.voice,
            self.eye_color,
            self.features_type,
            self.features_color,
            self.hair_color,
            self.clothing_color,
            self.skin_tone,
            self.hunter_rank,
            self.total_hr_points,
            self.funds,
            self.play_time_mh4g,
            self.play_time_mh4,
            self.equipped_weapon,
            self.equipped_armor,
            self.equipped_talismen)
    }
}
