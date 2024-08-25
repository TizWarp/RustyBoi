use sdl2::libc::DT_REG;

pub struct RegisterAddress {
    register: u8,
    index: RegisterIndex,
}

pub enum RegisterIndex {
    First,
    Last,
}

impl RegisterAddress {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Self {
                register: 0,
                index: RegisterIndex::First,
            },
            1 => Self {
                register: 0,
                index: RegisterIndex::Last,
            },
            2 => Self {
                register: 1,
                index: RegisterIndex::First,
            },
            3 => Self {
                register: 1,
                index: RegisterIndex::Last,
            },
            4 => Self {
                register: 2,
                index: RegisterIndex::First,
            },
            5 => Self {
                register: 2,
                index: RegisterIndex::Last,
            },
            6 => Self {
                register: 3,
                index: RegisterIndex::First,
            },
            7 => Self {
                register: 3,
                index: RegisterIndex::Last,
            },
            8 => Self {
                register: 4,
                index: RegisterIndex::First,
            },
            9 => Self {
                register: 4,
                index: RegisterIndex::Last,
            },
            10 => Self {
                register: 5,
                index: RegisterIndex::First,
            },
            11 => Self {
                register: 5,
                index: RegisterIndex::Last,
            },
            12 => Self {
                register: 6,
                index: RegisterIndex::First,
            },
            13 => Self {
                register: 6,
                index: RegisterIndex::Last,
            },
            14 => Self {
                register: 7,
                index: RegisterIndex::First,
            },
            15 => Self {
                register: 7,
                index: RegisterIndex::Last,
            },
            _ => Self {
                register: 0,
                index: RegisterIndex::First,
            },
        }
    }
}

#[derive(Debug)]
pub struct RegisterFile {
    registers: [[u8; 2]; 8],
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            registers: [[0; 2]; 8],
        }
    }

    pub fn read_byte(&self, register: RegisterAddress) -> u8 {
        match register.index {
            RegisterIndex::First => self.registers[register.register as usize][0],
            RegisterIndex::Last => self.registers[register.register as usize][1],
        }
    }

    pub fn write_byte(&mut self, register: RegisterAddress, byte: u8) {
        match register.index {
            RegisterIndex::First => self.registers[register.register as usize][0] = byte,
            RegisterIndex::Last => self.registers[register.register as usize][1] = byte,
        }
    }

    pub fn read_word(&self, register: RegisterAddress) -> u16 {
        u16::from_le_bytes(self.registers[register.register as usize])
    }

    pub fn write_word(&mut self, register: RegisterAddress, word: u16) {
        let bytes = word.to_le_bytes();
        self.registers[register.register as usize] = bytes;
    }
}

pub struct FlagRegister {
    pub compare: bool,
    pub carry: bool,
}

impl FlagRegister {
    pub fn new() -> Self {
        Self {
            compare: false,
            carry: false,
        }
    }
}
