#[derive(Debug)]
#[repr(C, packed)]
pub struct Weapon {
    // Equipment type (0x07=great sword, 0x08=sword and shield, 0x09=hammer...)
    equipment_type:          u8,
    // kinsect_level: (max 0x0B)
    kinsect_level:           u8,
    weapon_id:               u16,
    element_value:           u8,
    element_type:            u8,
    decoration_slot_1:       u16,
    decoration_slot_2:       u16,
    decoration_slot_3:       u16,
    sharpness:               u8,
    attack_affinity_defense: u8,
    upgrade_level:           u8,
    // special: hunting horn notes,
    //          gunlance shells,
    //          switchaxe phial,
    //          charge blade phial,
    //          kinsect type,
    //          other?
    special:                 u8,
    // details: Clean or rusted (bit 0),
    //          no glow or glow (bit 1),
    //          number of slots (bits 2-3)
    details:                 u8,
    rarity:                  u8,
    polish_requirements:     u8,
    // hone types: 0x00=none, 0x40=attack, 0x80=defense, 0xC0=life)
    hone_type:               u8,
    kinsect_power_growth:    u8,
    kinsect_stamina_growth:  u8,
    kinsect_speed_growth:    u8,
    kinsect_fire_growth:     u8,
    kinsect_water_growth:    u8,
    kinsect_thunder_growth:  u8,
    kinsect_ice_growth:      u8,
    kinsect_dragon_growth:   u8,
}

