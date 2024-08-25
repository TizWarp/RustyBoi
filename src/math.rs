use crate::{
    opcodes::{Opcode, OpcodeWidth},
    rusty_boi::RustyBoi,
};

impl RustyBoi {
    pub fn add_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1);
        let byte2 = self.read_reg_byte(reg2);
        let (byte, carry) = byte1.overflowing_add(byte2);
        self.write_reg_byte(reg3, byte);
        self.flag_register.carry = carry;
    }

    pub fn add_words(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let word1 = self.read_reg_word(reg1);
        let word2 = self.read_reg_word(reg2);
        let (word, carry) = word1.overflowing_add(word2);
        self.write_reg_word(reg3, word);
        self.flag_register.carry = carry;
    }

    pub fn add_signed_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1) as i8;
        let byte2 = self.read_reg_byte(reg2) as i8;
        let (byte, carry) = byte1.overflowing_add(byte2);
        self.write_reg_byte(reg3, byte as u8);
        self.flag_register.carry = carry;
    }

    pub fn add_signed_words(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let word1 = self.read_reg_word(reg1) as i16;
        let word2 = self.read_reg_word(reg2) as i16;
        let (word, carry) = word1.overflowing_add(word2);
        self.write_reg_word(reg3, word as u16);
        self.flag_register.carry = carry;
    }

    pub fn sub_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1);
        let byte2 = self.read_reg_byte(reg2);
        let (byte, carry) = byte1.overflowing_sub(byte2);
        self.write_reg_byte(reg3, byte);
        self.flag_register.carry = carry;
    }

    pub fn sub_words(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let word1 = self.read_reg_word(reg1);
        let word2 = self.read_reg_word(reg2);
        let (word, carry) = word1.overflowing_sub(word2);
        self.write_reg_word(reg3, word);
        self.flag_register.carry = carry;
    }

    pub fn sub_signed_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1) as i8;
        let byte2 = self.read_reg_byte(reg2) as i8;
        let (byte, carry) = byte1.overflowing_sub(byte2);
        self.write_reg_byte(reg3, byte as u8);
        self.flag_register.carry = carry;
    }

    pub fn sub_signed_words(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let word1 = self.read_reg_word(reg1) as i16;
        let word2 = self.read_reg_word(reg2) as i16;
        let (word, carry) = word1.overflowing_sub(word2);
        self.write_reg_word(reg3, word as u16);
        self.flag_register.carry = carry;
    }

    pub fn mul_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1);
        let byte2 = self.read_reg_byte(reg2);
        let (byte, carry) = byte1.overflowing_mul(byte2);
        self.write_reg_byte(reg3, byte);
        self.flag_register.carry = carry;
    }

    pub fn mul_words(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let word1 = self.read_reg_word(reg1);
        let word2 = self.read_reg_word(reg2);
        let (word, carry) = word1.overflowing_mul(word2);
        self.write_reg_word(reg3, word);
        self.flag_register.carry = carry;
    }

    pub fn mul_signed_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1) as i8;
        let byte2 = self.read_reg_byte(reg2) as i8;
        let (byte, carry) = byte1.overflowing_mul(byte2);
        self.write_reg_byte(reg3, byte as u8);
        self.flag_register.carry = carry;
    }

    pub fn mul_signed_words(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let word1 = self.read_reg_word(reg1) as i16;
        let word2 = self.read_reg_word(reg2) as i16;
        let (word, carry) = word1.overflowing_mul(word2);
        self.write_reg_word(reg3, word as u16);
        self.flag_register.carry = carry;
    }

    pub fn div_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1);
        let byte2 = self.read_reg_byte(reg2);
        let (byte, carry) = byte1.overflowing_div(byte2);
        self.write_reg_byte(reg3, byte);
        self.flag_register.carry = carry;
    }

    pub fn div_words(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let word1 = self.read_reg_word(reg1);
        let word2 = self.read_reg_word(reg2);
        let (word, carry) = word1.overflowing_div(word2);
        self.write_reg_word(reg3, word);
        self.flag_register.carry = carry;
    }

    pub fn div_signed_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1) as i8;
        let byte2 = self.read_reg_byte(reg2) as i8;
        let (byte, carry) = byte1.overflowing_div(byte2);
        self.write_reg_byte(reg3, byte as u8);
        self.flag_register.carry = carry;
    }

    pub fn div_signed_words(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let word1 = self.read_reg_word(reg1) as i16;
        let word2 = self.read_reg_word(reg2) as i16;
        let (word, carry) = word1.overflowing_div(word2);
        self.write_reg_word(reg3, word as u16);
        self.flag_register.carry = carry;
    }

    pub fn shr_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1);
        let byte2 = self.read_reg_byte(reg2);
        let (byte, carry) = byte1.overflowing_shr(byte2 as u32);
        self.write_reg_byte(reg3, byte);
        self.flag_register.carry = carry;
    }

    pub fn shr_words(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let word1 = self.read_reg_word(reg1);
        let word2 = self.read_reg_word(reg2);
        let (word, carry) = word1.overflowing_shr(word2 as u32);
        self.write_reg_word(reg3, word);
        self.flag_register.carry = carry;
    }

    pub fn mod_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1);
        let byte2 = self.read_reg_byte(reg2);
        let byte = byte1 % byte2;
        self.write_reg_byte(reg3, byte);
    }

    pub fn mod_words(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let word1 = self.read_reg_word(reg1);
        let word2 = self.read_reg_word(reg2);
        let word = word1 % word2;
        self.write_reg_word(reg3, word);
    }

    pub fn mod_signed_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let byte1 = self.read_reg_word(reg1) as i8;
        let byte2 = self.read_reg_word(reg2) as i8;
        let byte = byte1 % byte2;
        self.write_reg_byte(reg3, byte as u8);
    }

    pub fn mod_signed_words(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let word1 = self.read_reg_word(reg1) as i16;
        let word2 = self.read_reg_word(reg2) as i16;
        let word = word1 % word2;
        self.write_reg_word(reg3, word as u16);
    }

    pub fn shl_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1);
        let byte2 = self.read_reg_byte(reg2);
        let (byte, carry) = byte1.overflowing_shl(byte2 as u32);
        self.write_reg_byte(reg3, byte);
        self.flag_register.carry = carry;
    }

    pub fn shl_words(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let reg3 = self.parse_reg();
        let word1 = self.read_reg_word(reg1);
        let word2 = self.read_reg_word(reg2);
        let (word, carry) = word1.overflowing_shl(word2 as u32);
        self.write_reg_word(reg3, word);
        self.flag_register.carry = carry;
    }
}
