#[derive(Debug)]
#[repr(C, packed)]
pub struct Armor {
    // equipment_type: 0x01=chest
    //                 0x02=arms
    //                 0x03=waist
    //                 0x04=legs
    //                 0x05=head
    equipment_type:        u8,
    upgrade_level:         u8,
    equipment_id:          u16,
    // pigment: (16bit 565 BGR)
    pigment:               u16,
    decoration_slot_1:     u16,
    decoration_slot_2:     u16,
    decoration_slot_3:     u16,
    // elemental_resistances: max 0x27
    elemental_resistances: u8,
    // defence: max 0x1B
    defence:               u8,
    unknown_1:             u16,
    // details: Clean or rusted (bit 0),
    //          no glow or glow (bit 1),
    //          number of slots (bits 2-3)
    details:               u8,
    rarity:                u8,
    polish_requirements:   u8,
    // color_mode: 0x00=default
    //             0x40=custom
    //             0x80=rainbow
    color_mode:            u8,
    unknown_2:             u64,
}

