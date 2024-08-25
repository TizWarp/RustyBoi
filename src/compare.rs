use crate::rusty_boi::RustyBoi;

impl RustyBoi {
    pub fn eq_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1);
        let byte2 = self.read_reg_byte(reg2);
        self.flag_register.compare = byte1 == byte2;
    }

    pub fn eq_word(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let word1 = self.read_reg_word(reg1);
        let word2 = self.read_reg_word(reg2);
        self.flag_register.compare = word1 == word2;
    }

    pub fn neq_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1);
        let byte2 = self.read_reg_byte(reg2);
        self.flag_register.compare = byte1 != byte2;
    }

    pub fn neq_word(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let word1 = self.read_reg_word(reg1);
        let word2 = self.read_reg_word(reg2);
        self.flag_register.compare = word1 != word2;
    }

    pub fn les_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1);
        let byte2 = self.read_reg_byte(reg2);
        self.flag_register.compare = byte1 < byte2;
    }

    pub fn les_signed_bytes(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let byte1 = self.read_reg_byte(reg1) as i8;
        let byte2 = self.read_reg_byte(reg2) as i8;
        self.flag_register.compare = byte1 < byte2;
    }

    pub fn les_words(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let byte1 = self.read_reg_word(reg1);
        let byte2 = self.read_reg_word(reg2);
        self.flag_register.compare = byte1 < byte2;
    }

    pub fn les_signed_words(&mut self) {
        let reg1 = self.parse_reg();
        let reg2 = self.parse_reg();
        let byte1 = self.read_reg_word(reg1) as i16;
        let byte2 = self.read_reg_word(reg2) as i16;
        self.flag_register.compare = byte1 < byte2;
    }
}
