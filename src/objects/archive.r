#[derive(Debug)]
#[repr(C, packed)]
pub struct MsgIndexEntry {
    pub offset:u32,
    pub size:u32,
    unknown_1:u32,
}

impl MsgIndexEntry {
    fn size(&self) -> u32 {
        self.size * 2
    }
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct ArchiveHeader {
    magic: u32,
    format_version: u16,
    file_count: u16,
    unknown_1: u32,
}

#[repr(C, packed)]
pub struct ArchiveEntry {
    file_name: [u8; 64],
    file_type: u32,
    compressed_file_size: u32,
    decompressed_file_size: u32,
    file_offset: u32,
}

impl fmt::Debug for ArchiveEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,  r#"Entry: (
        file_name: {},
        file_type: {},
        file_offset: {},
        compressed_file_size: {},
        decompressed_file_size: {}
        )"#,
        self.file_name(),
        self.file_type,
        self.file_offset,
        self.compressed_file_size,
        self.decompressed_file_size())
    }
}

impl ArchiveEntry {
    fn file_name(&self) -> String {
        // trim matches to remove extra nulls from the buffer
        String::from(str::from_utf8(&self.file_name).unwrap().trim_matches('\0'))
    }

    fn file_name_unix(&self) -> String {
        //self.file_name()
        self.file_name().replace("\\", ".")
    }

    fn decompressed_file_size(&self) -> u32 {
        // The last 4 bytes of this are for unknown purposes
        // I'm not entirely sure how to mask of 4 bits in rush
        // so instead I'm just shifting forward 4 then backwards
        (self.decompressed_file_size << 4) >> 4
    }
}

