use std::mem;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io;
use std::collections::HashMap;

use flate2::{Decompress, Flush, Status, DataError};
use encoding::{Encoding, DecoderTrap};
use encoding::all::{UTF_16LE};
use regex::Regex;

use objects::archive::{ArchiveHeader, ArchiveEntry, MsgIndexEntry, MessageCollection};
use objects::equipment::{EquipmentType};

// const DECOMPRESSED_FILE_SIZE_MASK: u32 = 0b1111_0000_0000_0000_0000_0000_0000_0000;

enum MessageType {
    Name,
    Explanation,
    Empty,
}

fn file_name_to_equipment_type() -> Vec<(String, (EquipmentType, MessageType))> {
    let mappings = vec![
        ("HeadName",    (EquipmentType::Head,           MessageType::Name,        1)),
        ("HeadExp",     (EquipmentType::Head,           MessageType::Explanation, 1)),
        ("BodyName",    (EquipmentType::Chest,          MessageType::Name,        2)),
        ("BodyExp",     (EquipmentType::Chest,          MessageType::Explanation, 2)),
        ("ArmName",     (EquipmentType::Arms,           MessageType::Name,        3)),
        ("ArmExp",      (EquipmentType::Arms,           MessageType::Explanation, 3)),
        ("WaistName",   (EquipmentType::Waist,          MessageType::Name,        4)),
        ("WaistExp",    (EquipmentType::Waist,          MessageType::Explanation, 4)),
        ("LegName",     (EquipmentType::Legs,           MessageType::Name,        5)),
        ("LegExp",      (EquipmentType::Legs,           MessageType::Explanation, 5)),
        ("AcceName",    (EquipmentType::Talismen,       MessageType::Name,        6)),
        ("AcceExp",     (EquipmentType::Talismen,       MessageType::Explanation, 6)),
        ("LswordName",  (EquipmentType::GreatSword,     MessageType::Name,        7)),
        ("LswordExp",   (EquipmentType::GreatSword,     MessageType::Explanation, 7)),
        ("SwordName",   (EquipmentType::SwordAndShield, MessageType::Name,        8)),
        ("SwordExp",    (EquipmentType::SwordAndShield, MessageType::Explanation, 8)),
        ("HammerName",  (EquipmentType::Hammer,         MessageType::Name,        9)),
        ("HammerExp",   (EquipmentType::Hammer,         MessageType::Explanation, 9)),
        ("BowName",     (EquipmentType::Bow,            MessageType::Name,        10)),
        ("BowExp",      (EquipmentType::Bow,            MessageType::Explanation, 10)),
        ("WswordName",  (EquipmentType::DualBlades,     MessageType::Name,        11)),
        ("WswordExp",   (EquipmentType::DualBlades,     MessageType::Explanation, 11)),
        ("Hammer2Name", (EquipmentType::HuntingHorn,    MessageType::Name,        12)),
        ("Hammer2Exp",  (EquipmentType::HuntingHorn,    MessageType::Explanation, 12)),
        ("RodName",     (EquipmentType::InsectGlave,    MessageType::Name,        13)),
        ("RodExp",      (EquipmentType::InsectGlave,    MessageType::Explanation, 13)),
        ("GaxeName",    (EquipmentType::ChargeBlade,    MessageType::Name,        14)),
        ("GaxeExp",     (EquipmentType::ChargeBlade,    MessageType::Explanation, 14)),
        ("LanceName",   (EquipmentType::Lance,          MessageType::Name,        15)),
        ("LanceExp",    (EquipmentType::Lance,          MessageType::Explanation, 15)),
        ("LightName",   (EquipmentType::LightBowGun,    MessageType::Name,        16)),
        ("LightExp",    (EquipmentType::LightBowGun,    MessageType::Explanation, 16)),
        ("HeavyName",   (EquipmentType::HeavyBowGun,    MessageType::Name,        17)),
        ("HeavyExp",    (EquipmentType::HeavyBowGun,    MessageType::Explanation, 17)),
        ("Lsword2Name", (EquipmentType::LongSword,      MessageType::Name,        19)),
        ("Lsword2Exp",  (EquipmentType::LongSword,      MessageType::Explanation, 19)),
        ("AxeName",     (EquipmentType::SwitchAxe,      MessageType::Name,        20)),
        ("AxeExp",      (EquipmentType::SwitchAxe,      MessageType::Explanation, 20)),
        ("Lance2Name",  (EquipmentType::GunLance,       MessageType::Name,        21)),
        ("Lance2Exp",   (EquipmentType::GunLance,       MessageType::Explanation, 21)),
    ];
}

fn file_name_to_equipment_type_map() -> HashMap<String, (EquipmentType, MessageType, u8)> {
    let mappings = file_name_to_equipment_type();

    let h = HashMap::new();

    for (name, types) in mappings.iter() {
        h.insert(name, types);
    }

    h
}

fn read_header(f: &mut File) -> Result<ArchiveHeader, i32> {
    let mut buf: [u8; 12] = [0; 12];

    match f.read(&mut buf) {
        Ok(12) => {
            let h:ArchiveHeader = unsafe { mem::transmute(buf) };
            Ok(h)
        },
        Ok(_) => Err(1),
        Err(_) => Err(2),
    }
}

fn read_entry(f: &mut File) -> Result<ArchiveEntry, i32> {
    let mut buf: [u8; 80] = [0; 80];

    match f.read(&mut buf) {
        Ok(80) => {
            let e:ArchiveEntry = unsafe { mem::transmute(buf) };
            Ok(e)
        },
        Ok(_) => Err(1),
        Err(_) => Err(2),
    }
}

fn read_entries(f: &mut File, header: &ArchiveHeader) -> Vec<ArchiveEntry> {
    let mut entries = Vec::<ArchiveEntry>::with_capacity(header.file_count as usize);

    for _ in 0..header.file_count {
        entries.push(read_entry(f).unwrap())
    }

    entries
}

fn read_file_chunk(entry: &ArchiveEntry, f: &mut File) -> Vec<u8> {
    let mut b:Vec<u8> = vec![0; entry.compressed_file_size as usize];
    f.seek(SeekFrom::Start(entry.file_offset as u64)).unwrap();
    f.read(&mut b).unwrap();
    b
}

fn decompress_file_chunk(entry: &ArchiveEntry, bytes:&Vec<u8>) -> Vec<u8> {
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

fn write_entry(entry: &ArchiveEntry, chunk: &Vec<u8>, destination: &str) -> io::Result<()> {
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


pub fn decode_text_file(equipment_type_map:HashMap<&str, (EquipmentType, MessageType, u8)>, source:&str, source_name:&str) -> MessageCollection {
    println!("decoding: {} into {}", source, source_name.to_string());
    let mut f = File::open(source).unwrap();

    let (equipement_type, message_type, equipment_id) = match equipment_type_map.get(source_name) {
        Some((e_type, m_type, e_id)) => (e_type, m_type, e_id),
        None => (EquipmentType::Empty, MessageType::Empty, 0),
    };

    let mut messages:Vec<String> = Vec::new();
    for entry in read_index(&mut f) {
        match read_msg(&mut f, &entry) {
            Ok(msg) => messages.push(msg),
            _ => (),
        }
    }

    MessageCollection { source: source.to_string(),
                        source_name: source_name.to_string(),
                        messages: messages,
                        message_type: message_type,
                        equipement_type: equipement_type,
                        equipment_id: equipment_id }
}

pub fn decode_text_files(source:&str) -> Vec<MessageCollection> {
    let re = Regex::new(r"eng\.msg\.(.*)_eng").unwrap();
    let equipment_type_map = file_name_to_equipment_type_map();

    fs::read_dir(source).unwrap().filter_map(|entry| {
        let entry = entry.unwrap();

        let file_name = entry.file_name().into_string().unwrap();
        let path_string = entry.path().into_os_string().into_string().unwrap();

        re.captures(&file_name).map(|cap| {
            (path_string.to_string(), cap.at(1).unwrap())
        }).map(|(path_string, source_name)| {
            decode_text_file(&equipment_type_map, &path_string, source_name)
        })
    }).collect::<Vec<MessageCollection>>()
}

pub fn decompress(source:&str, destination:&str) {
    println!("Decompressing: {}", source);
    let mut f = File::open(source).unwrap();
    let header = read_header(&mut f).unwrap();
    let entries = read_entries(&mut f, &header);

    for entry in entries {
        let file_chunk = read_file_chunk(&entry, &mut f);
        let decompressed_chunk = decompress_file_chunk(&entry, &file_chunk);
        write_entry(&entry, &decompressed_chunk, destination).ok();
    }
}
