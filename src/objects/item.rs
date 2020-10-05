#[derive(Debug)]
#[repr(C, packed)]
pub struct Item {
    pub item_id: u16,
    pub count: u16
}
