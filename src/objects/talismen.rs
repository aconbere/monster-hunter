#[derive(Debug)]
#[repr(C, packed)]
pub struct Talismen {
    // equipment_type: 0x06=talismen
    equipment_type:    u8,
    number_of_slots:   u8,
    talismen_id:       u16,
    unknown_1:         u16,
    decoration_slot_1: u16,
    decoration_slot_2: u16,
    decoration_slot_3: u16,
    skill_1_id:        u16,
    skill_1_amount:    u16,
    skill_2_id:        u16,
    skill_2_amount:    u16,
    unknown_2:         u64
}

