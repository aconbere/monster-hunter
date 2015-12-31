#[derive(Debug)]
#[repr(C, packed)]
pub struct Weapon {
    // Equipment type (0x07=great sword, 0x08=sword and shield, 0x09=hammer...)
    pub equipment_type:          u8,
    // kinsect_level: (max 0x0B)
    pub kinsect_level:           u8,
    pub weapon_id:               u16,
    pub element_value:           u8,
    pub element_type:            u8,
    pub decoration_slot_1:       u16,
    pub decoration_slot_2:       u16,
    pub decoration_slot_3:       u16,
    pub sharpness:               u8,
    pub attack_affinity_defense: u8,
    pub upgrade_level:           u8,
    // special: hunting horn notes,
    //          gunlance shells,
    //          switchaxe phial,
    //          charge blade phial,
    //          kinsect type,
    //          other?
    pub special:                 u8,
    // details: Clean or rusted (bit 0),
    //          no glow or glow (bit 1),
    //          number of slots (bits 2-3)
    pub details:                 u8,
    pub rarity:                  u8,
    pub polish_requirements:     u8,
    // hone types: 0x00=none, 0x40=attack, 0x80=defense, 0xC0=life)
    pub hone_type:               u8,
    pub kinsect_power_growth:    u8,
    pub kinsect_stamina_growth:  u8,
    pub kinsect_speed_growth:    u8,
    pub kinsect_fire_growth:     u8,
    pub kinsect_water_growth:    u8,
    pub kinsect_thunder_growth:  u8,
    pub kinsect_ice_growth:      u8,
    pub kinsect_dragon_growth:   u8,
}
