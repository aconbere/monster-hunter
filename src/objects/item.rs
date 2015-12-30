#[derive(Debug)]
#[repr(C, packed)]
pub struct Item {
    item_id: u16,
    item_count: u16
}

