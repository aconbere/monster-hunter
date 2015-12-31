#[derive(Debug)]
#[repr(C, packed)]
pub struct Talismen {
    // equipment_type: 0x06=talismen
    pub equipment_type:    u8,
    pub number_of_slots:   u8,
    pub talismen_id:       u16,
    pub unknown_1:         u16,
    pub decoration_slot_1: u16,
    pub decoration_slot_2: u16,
    pub decoration_slot_3: u16,
    pub skill_1_id:        u16,
    pub skill_1_amount:    i16,
    pub skill_2_id:        u16,
    pub skill_2_amount:    i16,
    pub unknown_2:         u64
}

