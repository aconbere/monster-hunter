use std::fmt;

#[repr(C, packed)]
pub struct Palico {
    pub data: [u8; 232],
}

impl fmt::Debug for Palico {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Palico: (...)")
    }
}


