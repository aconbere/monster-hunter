use std::fmt;
use std::str;
use std::mem;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io;

use flate2::{Decompress, Flush, Status, DataError};
use encoding::{Encoding, DecoderTrap};
use encoding::all::{UTF_16LE};

// const DECOMPRESSED_FILE_SIZE_MASK: u32 = 0b1111_0000_0000_0000_0000_0000_0000_0000;

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

impl Entry {
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

fn read_header(f: &mut File) -> Result<Header, i32> {
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

fn read_entry(f: &mut File) -> Result<Entry, i32> {
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

fn read_entries(f: &mut File, header: &Header) -> Vec<Entry> {
    let mut entries = Vec::<Entry>::with_capacity(header.file_count as usize);

    for _ in 0..header.file_count {
        entries.push(read_entry(f).unwrap())
    }

    entries
}

fn read_file_chunk(entry: &Entry, f: &mut File) -> Vec<u8> {
    let mut b:Vec<u8> = vec![0; entry.compressed_file_size as usize];
    f.seek(SeekFrom::Start(entry.file_offset as u64)).unwrap();
    f.read(&mut b).unwrap();
    b
}

fn decompress_file_chunk(entry: &Entry, bytes:&Vec<u8>) -> Vec<u8> {
    let mut b:Vec<u8> = Vec::with_capacity(entry.decompressed_file_size() as usize);
    let mut d = Decompress::new(true);
    match d.decompress_vec(bytes, &mut b, Flush::Finish) {
        Ok(Status::Ok) =>        println!("decompressed successfully"),
        Ok(Status::BufError) =>  println!("decompress failed: BufError"),
        Ok(Status::StreamEnd) => println!("decompress finished"),
        Err(DataError(_)) =>     println!("decompress failed: DataError")
    };
    b
}

fn write_entry(entry: &Entry, chunk: &Vec<u8>, destination: &String) -> io::Result<()> {
    let out_file = format!("{}/{}", destination, entry.file_name_unix());

    match File::create(out_file) {
        Ok(mut f) => {
            try!(f.write_all(chunk));
        },
        Err(err) => {
            panic!(err)
        }
    }
    Ok(())
}

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

pub fn read_index_entry(f:&mut File) -> Result<MsgIndexEntry, u32> {
    let mut b: [u8; 12] = [0; 12];

    match f.read(&mut b) {
        Ok(12) => {
            let index:MsgIndexEntry = unsafe { mem::transmute(b) };
            Ok(index)
        },
        Ok(_) => Err(1),
        Err(_) => Err(2),
    }
}

pub fn read_index_offset(f:&mut File) -> Result<u32, u32> {
    let offset_start = 28;
    let mut b: [u8; 4] = [0; 4];

    f.seek(SeekFrom::Start(offset_start as u64)).unwrap();

    match f.read(&mut b) {
        Ok(4) => {
            let offset:u32 = unsafe { mem::transmute(b) };
            Ok(offset)
        },
        Ok(_) => Err(1),
        Err(_) => Err(2),
    }
}

pub fn read_index(f:&mut File) -> Vec<MsgIndexEntry> {
    let index_offset = read_index_offset(f).unwrap();
    f.seek(SeekFrom::Start(index_offset as u64)).unwrap();

    let first_index_entry = read_index_entry(f).unwrap();
    let size_of_index_entry = mem::size_of::<MsgIndexEntry>();

    let entry_count = (first_index_entry.offset - index_offset) / (size_of_index_entry as u32);

    let mut index = Vec::with_capacity(entry_count as usize);
    index.push(first_index_entry);

    for _ in 1..entry_count {
        index.push(read_index_entry(f).unwrap());
    }

    index
}

pub fn read_msg(f:&mut File, entry: &MsgIndexEntry) -> Result<String,u32> {
    let mut b:Vec<u8> = vec![0; entry.size() as usize];
    
    f.seek(SeekFrom::Start(entry.offset as u64)).unwrap();

    match f.read(&mut b) {
        Ok(_) => {
            let msg = UTF_16LE.decode(&b, DecoderTrap::Replace).unwrap();
            Ok(msg)
        },
        Err(_) => Err(1)
    }
}

pub fn decode(source:&String, _destination:&String) {
    let mut f = File::open(source).unwrap();

    let index:Vec<MsgIndexEntry> = read_index(&mut f);

    for i in index {
        println!("{}", read_msg(&mut f, &i).unwrap());
    }
}

pub fn decompress(source:&String, destination:&String) {
    println!("Decoding: {}", source);
    let mut f = File::open(source).unwrap();
    let header = read_header(&mut f).unwrap();
    let entries = read_entries(&mut f, &header);

    for entry in entries {
        let file_chunk = read_file_chunk(&entry, &mut f);
        let decompressed_chunk = decompress_file_chunk(&entry, &file_chunk);
        write_entry(&entry, &decompressed_chunk, destination).ok();
    }
}
