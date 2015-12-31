use std::fmt;
use std::str;
use std::mem;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
#[repr(C, packed)]
pub struct Header {
    magic: u32,
    format_version: u16,
    file_count: u16,
    unknown_1: u32,
}

#[repr(C, packed)]
pub struct Entry {
    file_name: [u8; 64],
    file_type: u32,
    compressed_file_size: u32,
    decompressed_file_size: u32,
    file_offset: u32,
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,  r#"Entry: (
        {},
        {}
        )"#, self.file_name(), self.file_offset)
    }
}

impl Entry {
    fn file_name(&self) -> String {
        // String::from_ut8(Vec::from(self.file_name)).unwrap()
        // String::from(str::from_utf8(&self.file_name).unwrap())
        String::from(str::from_utf8(&self.file_name).unwrap())
        // String::from_utf8_lossy(&self.file_name).to_owned()
    }
}

fn read_header(mut f: &File) -> Result<Header, i32> {
    let mut buf: [u8; 12] = [0; 12];

    match f.read(&mut buf) {
        Ok(12) => {
            let h:Header = unsafe { mem::transmute(buf) };
            Ok(h)
        },
        Ok(_) => Err(1),
        Err(_) => Err(2),
    }
}

fn read_entry(mut f: &File) -> Result<Entry, i32> {
    let mut buf: [u8; 80] = [0; 80];


    match f.read(&mut buf) {
        Ok(80) => {
            let e:Entry = unsafe { mem::transmute(buf) };
            Ok(e)
        },
        Ok(_) => Err(1),
        Err(_) => Err(2),
    }
}

pub fn decode(source:&String, _destination:&String) -> Vec<Entry> {
    let f = File::open(source).unwrap();

    let header = read_header(&f).unwrap();

    let mut entries = Vec::<Entry>::with_capacity(header.file_count as usize);

    for _ in 0..header.file_count {
        entries.push(read_entry(&f).unwrap())
    }

    entries
}
