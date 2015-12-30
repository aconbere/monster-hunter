use std::fmt;

#[repr(C, packed)]
pub struct GuildQuest {
    data: [u8; 304]
}

impl fmt::Debug for GuildQuest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GuildQuest: (...)")
    }
}

