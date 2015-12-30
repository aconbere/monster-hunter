use std::fmt;

#[repr(C, packed)]
pub struct Expedition {
    pub data: [u8; 264],
}

impl fmt::Debug for Expedition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expedition: (...)")
    }
}


