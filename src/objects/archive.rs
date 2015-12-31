#[derive(Debug)]
#[repr(C, packed)]
pub struct Header {
    magic: u32,
    format_version: u16,
    file_count: u16,
    unknown_1: u32,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct Entry {
    file_name: [u8; 64],
    file_type: u32,
    compressed_file_size: u32,
    decompressed_file_size: u32,
    file_offset: u32,
}

fn extract(source) {
    let mut f = File::open(source).unwrap();
    let mut header_buf: [u8, 12] = [0; 12];

    match f.read(&mut header_buf) {
        Ok(12) => {
            let archive: Header = unsafe {
                mem::transmute(header_buf)
            };
            println!("Header: {}", archive)
        }
        _ => panic!("read failed")
    }
}
