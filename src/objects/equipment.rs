use std::fmt;
use std::mem;

use objects::weapon::Weapon;
use objects::armor::Armor;
use objects::talismen::Talismen;

enum EquipmentClass {
    Weapon,
    Armor,
    Talismen,
    Empty
}

pub enum DecodedEquipmentClass {
    Weapon(Weapon),
    Armor(Armor),
    Talismen(Talismen),
    Empty
}

#[repr(C, packed)]
pub struct Equipment {
    pub buffer: [u8; 28],
}

impl Equipment {
    fn equipment_class(&self) -> EquipmentClass {
        match self.buffer[0] {
            0 =>        EquipmentClass::Empty,
            1 ... 5 =>  EquipmentClass::Armor,
            6 =>        EquipmentClass::Talismen,
            7 ... 15 => EquipmentClass::Weapon,
            _ =>        EquipmentClass::Empty
        }
    }


    pub fn decode(&self) -> DecodedEquipmentClass {
        match self.equipment_class(){
            EquipmentClass::Armor => {
                let result: Armor = unsafe {
                    mem::transmute(self.buffer)
                };
                DecodedEquipmentClass::Armor(result)
            },
            EquipmentClass::Weapon => {
                let result: Weapon = unsafe {
                    mem::transmute(self.buffer)
                };
                DecodedEquipmentClass::Weapon(result)
            },
            EquipmentClass::Talismen => {
                let result: Talismen = unsafe {
                    mem::transmute(self.buffer)
                };
                DecodedEquipmentClass::Talismen(result)
            },
            EquipmentClass::Empty => {
                DecodedEquipmentClass::Empty
            }
        }

    }
}

impl fmt::Debug for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.decode() {
            DecodedEquipmentClass::Armor(armor) => write!(f, "{:#?}", armor),
            DecodedEquipmentClass::Weapon(weapon) => write!(f, "{:#?}", weapon),
            DecodedEquipmentClass::Talismen(talismen) => write!(f, "{:#?}", talismen),
            DecodedEquipmentClass::Empty => write!(f, "{}", "Empty")
        }
    }
}
