#[derive(Debug)]
#[repr(C, packed)]
pub struct Armor {
    // equipment_type: 0x01=chest
    //                 0x02=arms
    //                 0x03=waist
    //                 0x04=legs
    //                 0x05=head
    pub equipment_type:        u8,
    pub upgrade_level:         u8,
    pub armor_id:              u16,
    // pigment: (16bit 565 BGR)
    pub pigment:               u16,
    pub decoration_slot_1:     u16,
    pub decoration_slot_2:     u16,
    pub decoration_slot_3:     u16,
    // elemental_resistances: max 0x27 (39)
    pub elemental_resistances: u8,
    // defence: max 0x1B (27)
    pub defence:               u8,
    pub unknown_1:             u16,
    // details: Clean or rusted (bit 0),
    //          no glow or glow (bit 1),
    //          number of slots (bits 2-3)
    pub details:               u8,
    pub rarity:                u8,
    pub polish_requirements:   u8,
    // color_mode: 0x00=default
    //             0x40=custom
    //             0x80=rainbow
    pub color_mode:            u8,
    pub unknown_2:             u64,
}
