use crate::{
    opcodes::{Opcode, OpcodeWidth},
    register_file::{FlagRegister, RegisterAddress, RegisterFile},
};

const STACK_START: u16 = 0x00ff;

pub struct RustyBoi {
    sp: u16,
    pc: u16,
    register_file: RegisterFile,
    prog_mem: [u8; 65536],
    pub flag_register: FlagRegister,
    pub memory: [u8; 65536],
    ret_ptr: Vec<u16>,
}

impl RustyBoi {
    pub fn new() -> Self {
        Self {
            memory: [0; 65536],
            prog_mem: [0; 65536],
            register_file: RegisterFile::new(),
            flag_register: FlagRegister::new(),
            sp: STACK_START,
            ret_ptr: Vec::new(),
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        //println!("Runnng");
        if self.pc == u16::MAX {
            return;
        }
        loop {
            //println!(
            //     "{}",
            //     self.register_file.read_byte(RegisterAddress::from_byte(1))
            // );
            //    println!("Loading file");

            //println!("{:?}", self.pc);
            let opcode = Opcode::from_byte(self.read_prog_byte());
            //println!("{:?} @ {} ret ptr len is {}", opcode, self.pc, self.ret_ptr.len());
            match opcode {
                Opcode::JMP => self.jmp(),
                Opcode::CJMP => self.tjmp(),
                Opcode::NJMP => self.njmp(),
                Opcode::MOV { width } => match width {
                    OpcodeWidth::Byte => self.mov_byte(),
                    OpcodeWidth::Word => self.mov_word(),
                },
                Opcode::LOAD { width } => match width {
                    OpcodeWidth::Byte => self.load_byte(),
                    OpcodeWidth::Word => self.load_word(),
                },
                Opcode::STORE { width } => match width {
                    OpcodeWidth::Byte => self.store_byte(),
                    OpcodeWidth::Word => self.store_word(),
                },
                Opcode::READ { width } => match width {
                    OpcodeWidth::Byte => self.read_byte(),
                    OpcodeWidth::Word => self.read_word(),
                },
                Opcode::PUSH { width } => match width {
                    OpcodeWidth::Byte => self.push_byte(),
                    OpcodeWidth::Word => self.push_word(),
                },
                Opcode::POP { width } => match width {
                    OpcodeWidth::Byte => self.pop_byte(),
                    OpcodeWidth::Word => self.pop_word(),
                },
                Opcode::EQ { width } => match width {
                    OpcodeWidth::Byte => self.eq_bytes(),
                    OpcodeWidth::Word => self.eq_word(),
                },
                Opcode::NEQ { width } => match width {
                    OpcodeWidth::Byte => self.neq_bytes(),
                    OpcodeWidth::Word => self.neq_word(),
                },
                Opcode::LES { width, signed } => match (width, signed) {
                    (OpcodeWidth::Word, true) => self.les_signed_words(),
                    (OpcodeWidth::Word, false) => self.les_words(),
                    (OpcodeWidth::Byte, true) => self.les_signed_bytes(),
                    (OpcodeWidth::Byte, false) => self.les_bytes(),
                },
                Opcode::ADD { width, signed } => match (width, signed) {
                    (OpcodeWidth::Word, true) => self.add_signed_words(),
                    (OpcodeWidth::Word, false) => self.add_words(),
                    (OpcodeWidth::Byte, true) => self.add_signed_bytes(),
                    (OpcodeWidth::Byte, false) => self.add_bytes(),
                },
                Opcode::SUB { width, signed } => match (width, signed) {
                    (OpcodeWidth::Word, true) => self.sub_signed_words(),
                    (OpcodeWidth::Word, false) => self.sub_words(),
                    (OpcodeWidth::Byte, true) => self.sub_signed_bytes(),
                    (OpcodeWidth::Byte, false) => self.sub_bytes(),
                },
                Opcode::MUL { width, signed } => match (width, signed) {
                    (OpcodeWidth::Word, true) => self.mul_signed_words(),
                    (OpcodeWidth::Word, false) => self.mul_words(),
                    (OpcodeWidth::Byte, true) => self.mul_signed_bytes(),
                    (OpcodeWidth::Byte, false) => self.mul_bytes(),
                },
                Opcode::DIV { width, signed } => match (width, signed) {
                    (OpcodeWidth::Word, true) => self.div_signed_words(),
                    (OpcodeWidth::Word, false) => self.div_words(),
                    (OpcodeWidth::Byte, true) => self.div_signed_bytes(),
                    (OpcodeWidth::Byte, false) => self.div_bytes(),
                },
                Opcode::MOD { width, signed } => match (width, signed) {
                    (OpcodeWidth::Word, true) => self.mod_signed_words(),
                    (OpcodeWidth::Word, false) => self.mod_words(),
                    (OpcodeWidth::Byte, true) => self.mod_signed_bytes(),
                    (OpcodeWidth::Byte, false) => self.mod_bytes(),
                },
                Opcode::SHR { width } => match width {
                    OpcodeWidth::Byte => self.shr_bytes(),
                    OpcodeWidth::Word => self.shr_words(),
                },
                Opcode::SHL { width } => match width {
                    OpcodeWidth::Byte => self.shl_bytes(),
                    OpcodeWidth::Word => self.shl_words(),
                },
                Opcode::RET => self.ret(),
                Opcode::DRAW => return,
                Opcode::RDRAW => {
                    self.pc = 0;
                    return;
                }
                Opcode::JDRAW => {
                    let dest = self.read_prog_word();
                    self.jump(dest);
                    return;
                }
                Opcode::CALL => self.call_func(),
                Opcode::CCALL => self.ccall_func(),
                Opcode::NCALL => self.ncall_func(),
                Opcode::ILG => panic!("Illegal opcode"),
            }
        }
    }

    pub fn ret(&mut self) {
        let dest = self.ret_ptr.pop().unwrap();
        self.pc = dest;
    }

    pub fn call_func(&mut self) {
        let dest = self.read_prog_word();
        self.ret_ptr.push(self.pc);
        self.jump(dest);
    }

    pub fn ccall_func(&mut self) {
        let dest = self.read_prog_word();
        if self.flag_register.compare {
            self.ret_ptr.push(self.pc);
            self.jump(dest);
        }
    }

    pub fn ncall_func(&mut self) {
        let dest = self.read_prog_word();

        if !self.flag_register.compare {
            self.ret_ptr.push(self.pc);
            self.jump(dest);
        }
    }

    pub fn parse_reg(&mut self) -> RegisterAddress {
        RegisterAddress::from_byte(self.read_prog_byte())
    }

    pub fn read_prog_byte(&mut self) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        self.prog_mem[self.pc.wrapping_sub(1) as usize]
    }

    pub fn read_prog_word(&mut self) -> u16 {
        let bytes: [u8; 2] = [self.read_prog_byte(), self.read_prog_byte()];
        u16::from_le_bytes(bytes)
    }

    pub fn read_reg_byte(&mut self, reg: RegisterAddress) -> u8 {
        self.register_file.read_byte(reg)
    }

    pub fn read_reg_word(&mut self, reg: RegisterAddress) -> u16 {
        self.register_file.read_word(reg)
    }

    pub fn write_reg_byte(&mut self, reg: RegisterAddress, byte: u8) {
        self.register_file.write_byte(reg, byte);
    }

    pub fn write_reg_word(&mut self, reg: RegisterAddress, word: u16) {
        self.register_file.write_word(reg, word);
    }

    pub fn write_memory_word(&mut self, address: u16, word: u16) {
        let bytes = word.to_le_bytes();
        self.write_memory_byte(address, bytes[0]);
        self.write_memory_byte(address + 1, bytes[1]);
    }

    pub fn write_memory_byte(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }

    pub fn read_memory_byte(&mut self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_memory_word(&mut self, address: u16) -> u16 {
        let bytes: [u8; 2] = [
            self.read_memory_byte(address),
            self.read_memory_byte(address + 1),
        ];
        u16::from_le_bytes(bytes)
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.prog_mem[0..program.len()].copy_from_slice(&program);
    }

    pub fn load_byte(&mut self) {
        let reg = self.parse_reg();
        let byte = self.read_prog_byte();
        self.write_reg_byte(reg, byte);
    }

    pub fn load_word(&mut self) {
        let reg = self.parse_reg();
        let word = self.read_prog_word();
        self.write_reg_word(reg, word);
    }

    pub fn mov_byte(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let byte = self.read_reg_byte(reg1);
        self.write_reg_byte(reg2, byte);
    }

    pub fn mov_word(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let byte = self.read_reg_word(reg1);
        self.write_reg_word(reg2, byte);
    }

    pub fn store_byte(&mut self) {
        let addr = self.read_reg_word(RegisterAddress::from_byte(15));
        let reg = self.parse_reg();
        let byte = self.read_reg_byte(reg);
        self.write_memory_byte(addr, byte);
    }

    pub fn store_word(&mut self) {
        let addr = self.read_reg_word(RegisterAddress::from_byte(15));
        let reg = self.parse_reg();
        let word = self.read_reg_word(reg);
        self.write_memory_word(addr, word);
    }

    pub fn read_byte(&mut self) {
        let addr = self.read_reg_word(RegisterAddress::from_byte(15));
        let reg = self.parse_reg();
        let byte = self.read_memory_byte(addr);
        self.write_reg_byte(reg, byte);
    }

    pub fn read_word(&mut self) {
        let addr = self.read_reg_word(RegisterAddress::from_byte(15));
        let reg = self.parse_reg();
        let byte = self.read_memory_word(addr);
        self.write_reg_word(reg, byte);
    }

    pub fn jump(&mut self, address: u16) {
        self.pc = address;
    }

    pub fn jmp(&mut self) {
        let address = self.read_prog_word();
        self.jump(address);
    }

    pub fn tjmp(&mut self) {
        let address = self.read_prog_word();
        if self.flag_register.compare {
            self.jump(address);
        }
    }

    pub fn njmp(&mut self) {
        let address = self.read_prog_word();
        if !self.flag_register.compare {
            self.jump(address);
        }
    }

    pub fn push_byte(&mut self) {
        let reg = self.parse_reg();
        let byte = self.read_reg_byte(reg);
        self.write_memory_byte(self.sp, byte);
        match self.sp.checked_sub(1) {
            Some(num) => self.sp = num,
            None => self.sp = STACK_START,
        }
    }

    pub fn push_word(&mut self) {
        let reg = self.parse_reg();
        let word = self.read_reg_word(reg);
        self.write_memory_word(self.sp, word);
        match self.sp.checked_sub(2) {
            Some(num) => self.sp = num,
            None => self.sp = STACK_START,
        }
    }

    pub fn pop_byte(&mut self) {
        let reg = self.parse_reg();
        let byte = self.read_memory_byte(self.sp);
        if self.sp + 1 > STACK_START {
            self.sp = STACK_START;
        } else {
            self.sp += 1;
        }
        self.write_reg_byte(reg, byte);
    }

    pub fn pop_word(&mut self) {
        let reg = self.parse_reg();
        let byte = self.read_memory_word(self.sp);
        if self.sp + 2 > STACK_START {
            self.sp = STACK_START;
        } else {
            self.sp += 2;
        }
        self.write_reg_word(reg, byte);
    }
}
