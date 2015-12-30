use std::fmt;

#[repr(C, packed)]
pub struct Palico {
    pub name: [u16; 12],
    pub id: u64,
    pub coat_color: u32,
    pub clothing_color: u32,
    pub coat_type: u8,
    pub clothing_type: u8,
    pub eyes_type: u8,
    pub ears_type: u8,
    pub tail_type: u8,
    pub voice: u8,
    pub forte: u8,

    pub unknown_1: u8,
    pub unknown_2: u16,
    pub unknown_3: u16,

    pub weapon_type: u16,
    pub weapon_id: u16,
    pub chest_type: u16,
    pub chest_id: u16,
    pub head_type: u16,
    pub head_id: u16,
    pub equipment_box_index_weapon: u16,
    pub equipment_box_index_chest: u16,
    pub equipment_box_index_head: u16,

    pub unknown_4: u16,

    pub enthusiasm: u8,
    pub default_greeting: u8,
    pub custom_greeting: [u8; 30],

    pub unknown_5: u16,

    pub forte_skill_1: u8,
    pub forte_skill_2: u8,
    pub forte_skill_3: u8,
    pub forte_skill_4: u8,

    pub meownster_hunter_skill: u8,
    pub casting_machine_skill: u8,
    pub total_experience: u32,

    pub unknown_6: u32,

    pub namegiver_id: u64,
    pub namegiver_name: [u8; 24],

    pub unknown_7: [u8; 24],

    pub former_master_id: u64,
    pub former_master_name: [u8; 24],

    pub unknown_8: [u8; 24],
}

impl Palico {
    fn name(&self) -> String {
        String::from_utf16(&self.name).unwrap()
    }
}

impl fmt::Debug for Palico {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"Palico: (
        name: {})"#, self.name())
    }
}


