use std::{collections::HashMap, fmt::format, fs::File, io::Read, str::Chars};

#[derive(PartialEq, Eq)]
enum Width {
    Byte,
    Word,
}

pub struct Assembler {
    pc: u16,
    line_num: usize,
    labels: HashMap<String, u16>,
    constants: HashMap<String, u16>,
    program: Vec<u8>,
    file_string: String,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            pc: 0,
            line_num: 0,
            labels: HashMap::new(),
            constants: HashMap::new(),
            program: Vec::new(),
            file_string: "".to_string(),
        }
    }
    pub fn load_file(&mut self, file: &str) -> Result<(), String> {
        let mut file_handle = match File::open(file) {
            Ok(handle) => handle,
            Err(error) => return Err(format!("Error opening file {} \n{}", file, error)),
        };

        match file_handle.read_to_string(&mut self.file_string) {
            Ok(_) => Ok(()),
            Err(error) => Err(format!("Error reading file {} \n{}", file, error)),
        }
    }

    pub fn pre_assembly(&mut self) -> Result<(), String> {
        let lines: Vec<String> = self
            .file_string
            .lines()
            .filter(|x| !x.contains("//"))
            .map(|x| x.to_string())
            .collect();

        for line in lines.iter() {
            if line.len() == 0 {
                continue;
            }
            let args: Vec<&str> = line.split(" ").collect();

            match args[0] {
                "JMP" => self.pc += 3,
                "CJMP" => self.pc += 3,
                "NJMP" => self.pc += 3,
                "RET" => self.pc += 1,
                "DRAW" => self.pc += 1,
                "RDRAW" => self.pc += 1,
                "JDRAW" => self.pc += 3,
                "MOV" => self.pc += 3,
                "STORE" => self.pc += 2,
                "READ" => self.pc += 2,
                "POP" => self.pc += 2,
                "PUSH" => self.pc += 2,
                "EQ" => self.pc += 3,
                "NEQ" => self.pc += 3,
                "LES" => self.pc += 3,
                "ADD" => self.pc += 4,
                "SUB" => self.pc += 4,
                "MUL" => self.pc += 4,
                "DIV" => self.pc += 4,
                "MOD" => self.pc += 4,
                "LESi" => self.pc += 3,
                "ADDi" => self.pc += 4,
                "SUBi" => self.pc += 4,
                "MULi" => self.pc += 4,
                "DIVi" => self.pc += 4,
                "MODi" => self.pc += 4,
                "SHR" => self.pc += 4,
                "SHL" => self.pc += 4,
                "CALL" => self.pc += 3,
                "CCALL" => self.pc += 3,
                "NCALL" => self.pc += 3,
                "const" => match self.parse_const(args[1], args[2]) {
                    Ok(_) => (),
                    Err(error) => {
                        return Err(format!(
                            "Invalid nunber constant {} on line {} \n {}",
                            args[1], self.line_num, error
                        ))
                    }
                },
                _ => {
                    if args[0] == "LOAD" {
                        match self.parse_register(args[1]) {
                            Ok((_, width)) => match width {
                                Width::Byte => self.pc += 3,
                                Width::Word => self.pc += 4,
                            },
                            Err(error) => return Err(error),
                        }
                        continue;
                    }

                    if args[0].contains(":") {
                        match self.parse_label(args[0]) {
                            Ok(_) => continue,
                            Err(error) => return Err(error),
                        }
                    }

                    return Err(format!(
                        "Uknown opcode {} on line {}",
                        args[0], self.line_num
                    ));
                }
            }
            self.line_num += 1;
        }

        Ok(())
    }

    fn write_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }

    fn write_word(&mut self, word: u16) {
        let bytes = word.to_le_bytes();
        self.write_byte(bytes[0]);
        self.write_byte(bytes[1]);
    }

    fn parse_byte_hex(&mut self, number_str: &str) -> Result<u8, String> {
        let striped_string = match number_str.strip_prefix("0x") {
            Some(str) => str,
            None => {
                return Err(format!(
                    "Malformated number {} on line {}",
                    number_str, self.line_num
                ))
            }
        };

        match u8::from_str_radix(striped_string, 16) {
            Ok(num) => return Ok(num),
            Err(_) => (),
        }

        match i8::from_str_radix(striped_string, 16) {
            Ok(num) => return Ok(num as u8),
            Err(error) => {
                return Err(format!(
                    "Error parsing hexadecimal number {} on line {} /n{}",
                    number_str,
                    self.line_num,
                    error.to_string()
                ))
            }
        }
    }

    fn find_label(&mut self, label_str: &str) -> Result<u16, String> {
        match self.labels.get(label_str) {
            Some(num) => Ok(*num),
            None => Err(format!(
                "Uknown label {} on line {}",
                label_str, self.line_num
            )),
        }
    }

    fn find_constant(&mut self, const_str: &str) -> Result<u16, String> {
        match self.constants.get(const_str) {
            Some(num) => Ok(*num),
            None => Err(format!(
                "Uknown constant {} on line {}",
                const_str, self.line_num
            )),
        }
    }

    fn parse_label(&mut self, label_str: &str) -> Result<(), String> {
        let label = match label_str.strip_suffix(":") {
            Some(label) => label,
            None => {
                return Err(format!(
                    "Malformated label {} on line {}",
                    label_str, self.line_num
                ))
            }
        };
        self.labels.insert(label.to_string(), self.pc);
        Ok(())
    }

    fn parse_const(&mut self, const_str: &str, val: &str) -> Result<(), String> {
        if let Some(num_type) = Self::get_imm_type(val) {
            match num_type {
                ImmType::Decimal => match self.parse_word_dec(val) {
                    Ok(val) => {
                        self.constants.insert(const_str.to_string(), val);
                    }
                    Err(error) => return Err(error),
                },
                ImmType::Hex => match self.parse_word_hex(val) {
                    Ok(val) => {
                        self.constants.insert(const_str.to_string(), val);
                    }
                    Err(error) => return Err(error),
                },
            }
        }
        Ok(())
    }

    fn parse_byte_dec(&mut self, number_str: &str) -> Result<u8, String> {
        let striped_string = match number_str.strip_prefix("#") {
            Some(str) => str,
            None => {
                return Err(format!(
                    "Malformated number {} on line {}",
                    number_str, self.line_num
                ))
            }
        };

        match u8::from_str_radix(striped_string, 10) {
            Ok(num) => {
                println!("{}", num);
                return Ok(num);
            }
            Err(_) => (),
        }

        match i8::from_str_radix(striped_string, 10) {
            Ok(num) => Ok(num as u8),
            Err(error) => {
                return Err(format!(
                    "Error parsing decimal number {} on line {} /n{}",
                    number_str,
                    self.line_num,
                    error.to_string()
                ))
            }
        }
    }

    fn parse_word_hex(&mut self, number_str: &str) -> Result<u16, String> {
        let striped_string = match number_str.strip_prefix("0x") {
            Some(str) => str,
            None => {
                return Err(format!(
                    "Malformated number {} on line {}",
                    number_str, self.line_num
                ))
            }
        };

        match u16::from_str_radix(striped_string, 16) {
            Ok(num) => return Ok(num),
            Err(_) => (),
        }

        match i16::from_str_radix(striped_string, 16) {
            Ok(num) => Ok(num as u16),
            Err(error) => {
                return Err(format!(
                    "Error parsing hexadecimal number {} on line {} /n{}",
                    number_str,
                    self.line_num,
                    error.to_string()
                ))
            }
        }
    }

    fn parse_word_dec(&mut self, number_str: &str) -> Result<u16, String> {
        let striped_string = match number_str.strip_prefix("#") {
            Some(str) => str,
            None => {
                return Err(format!(
                    "Malformated number {} on line {}",
                    number_str, self.line_num
                ))
            }
        };

        match u16::from_str_radix(striped_string, 10) {
            Ok(num) => {
                println!("{}", num);
                return Ok(num);
            }
            Err(_) => (),
        }

        match i16::from_str_radix(striped_string, 10) {
            Ok(num) => Ok(num as u16),
            Err(error) => {
                return Err(format!(
                    "Error parsing decimal number {} on line {} /n{}",
                    number_str,
                    self.line_num,
                    error.to_string()
                ))
            }
        }
    }

    fn parse_register(&mut self, register_str: &str) -> Result<(u8, Width), String> {
        let register_chars: Vec<char> = register_str.chars().collect();

        let reg_num = match u8::from_str_radix(register_chars[1].to_string().as_str(), 10) {
            Ok(num) => num,
            Err(error) => {
                return Err(format!(
                    "Invalid register {} on line {} \n{}",
                    register_str, self.line_num, error
                ))
            }
        };

        if register_chars.len() == 2 {
            match reg_num {
                0 => return Ok((0, Width::Word)),
                1 => return Ok((2, Width::Word)),
                2 => return Ok((4, Width::Word)),
                3 => return Ok((6, Width::Word)),
                4 => return Ok((8, Width::Word)),
                5 => return Ok((10, Width::Word)),
                6 => return Ok((12, Width::Word)),
                7 => return Ok((14, Width::Word)),
                _ => {
                    return Err(format!(
                        "Invalid register {} on line {}",
                        register_str, self.line_num
                    ))
                }
            }
        } else {
            match (reg_num, register_chars[2]) {
                (0, 'a') => return Ok((0, Width::Byte)),
                (0, 'b') => return Ok((1, Width::Byte)),
                (1, 'a') => return Ok((2, Width::Byte)),
                (1, 'b') => return Ok((3, Width::Byte)),
                (2, 'a') => return Ok((4, Width::Byte)),
                (2, 'b') => return Ok((5, Width::Byte)),
                (3, 'a') => return Ok((6, Width::Byte)),
                (3, 'b') => return Ok((7, Width::Byte)),
                (4, 'a') => return Ok((8, Width::Byte)),
                (4, 'b') => return Ok((9, Width::Byte)),
                (5, 'a') => return Ok((10, Width::Byte)),
                (5, 'b') => return Ok((11, Width::Byte)),
                (6, 'a') => return Ok((12, Width::Byte)),
                (6, 'b') => return Ok((13, Width::Byte)),
                (7, 'a') => return Ok((14, Width::Byte)),
                (7, 'b') => return Ok((15, Width::Byte)),
                _ => {
                    return Err(format!(
                        "Invalid register {} on line {}",
                        register_str, self.line_num
                    ))
                }
            }
        }
    }

    fn get_imm_type(num_str: &str) -> Option<ImmType> {
        if num_str.contains("#") {
            return Some(ImmType::Decimal);
        } else if num_str.contains("0x") {
            return Some(ImmType::Hex);
        }

        None
    }
}

impl Assembler {
    pub fn assemble(&mut self) -> Result<Vec<u8>, String> {
        self.line_num = 0;

        let lines: Vec<String> = self
            .file_string
            .lines()
            .filter(|x| !x.contains("//"))
            .map(|x| x.to_string())
            .collect();

        for line in lines.iter() {
            if line.len() == 0 {
                continue;
            }
            let args: Vec<&str> = line.split(" ").collect();

            match args[0] {
                "LOAD" => {
                    let reg = match self.parse_register(args[1]) {
                        Ok(reg) => reg,
                        Err(error) => return Err(error),
                    };

                    match reg.1 {
                        Width::Byte => {
                            self.write_byte(0);
                            self.write_byte(reg.0);
                            if let Some(num_type) = Self::get_imm_type(args[2]) {
                                match num_type {
                                    ImmType::Decimal => match self.parse_byte_dec(args[2]) {
                                        Ok(val) => self.write_byte(val),
                                        Err(error) => return Err(error),
                                    },
                                    ImmType::Hex => match self.parse_byte_hex(args[2]) {
                                        Ok(val) => self.write_byte(val),
                                        Err(error) => return Err(error),
                                    },
                                }
                            }
                        }
                        Width::Word => {
                            self.write_byte(1);
                            self.write_byte(reg.0);
                            if let Some(num_type) = Self::get_imm_type(args[2]) {
                                match num_type {
                                    ImmType::Decimal => match self.parse_word_dec(args[2]) {
                                        Ok(val) => self.write_word(val),
                                        Err(error) => return Err(error),
                                    },
                                    ImmType::Hex => match self.parse_word_hex(args[2]) {
                                        Ok(val) => self.write_word(val),
                                        Err(error) => return Err(error),
                                    },
                                }
                            } else {
                                match self.find_constant(args[2]) {
                                    Ok(val) => self.write_word(val),
                                    Err(_) => {
                                        return Err(format!(
                                            "Uknown constant {} on line {}",
                                            args[2], self.line_num
                                        ))
                                    }
                                }
                            }
                        }
                    }
                }
                "MOV" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num,
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(2),
                        Width::Word => self.write_byte(3),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                }
                "STORE" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    match width1 {
                        Width::Byte => self.write_byte(4),
                        Width::Word => self.write_byte(5),
                    }

                    self.write_byte(reg1);
                }

                "READ" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    match width1 {
                        Width::Byte => self.write_byte(6),
                        Width::Word => self.write_byte(7),
                    }

                    self.write_byte(reg1);
                }
                "PUSH" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    match width1 {
                        Width::Byte => self.write_byte(8),
                        Width::Word => self.write_byte(9),
                    }

                    self.write_byte(reg1);
                }
                "POP" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    match width1 {
                        Width::Byte => self.write_byte(45),
                        Width::Word => self.write_byte(46),
                    }

                    self.write_byte(reg1);
                }
                "EQ" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num,
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(14),
                        Width::Word => self.write_byte(15),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                }
                "NEQ" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num,
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(43),
                        Width::Word => self.write_byte(44),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                }
                "JMP" => {
                    self.write_byte(10);
                    if let Some(imm_type) = Self::get_imm_type(args[1]) {
                        match imm_type {
                            ImmType::Decimal => match self.parse_word_dec(args[1]) {
                                Ok(val) => self.write_word(val),
                                Err(error) => return Err(error),
                            },
                            ImmType::Hex => match self.parse_word_hex(args[1]) {
                                Ok(val) => self.write_word(val),
                                Err(error) => return Err(error),
                            },
                        }
                    } else {
                        match self.find_label(args[1]) {
                            Ok(val) => self.write_word(val),
                            Err(error) => return Err(error),
                        }
                    }
                }
                "CJMP" => {
                    self.write_byte(11);
                    if let Some(imm_type) = Self::get_imm_type(args[1]) {
                        match imm_type {
                            ImmType::Decimal => match self.parse_word_dec(args[1]) {
                                Ok(val) => self.write_word(val),
                                Err(error) => return Err(error),
                            },
                            ImmType::Hex => match self.parse_word_hex(args[1]) {
                                Ok(val) => self.write_word(val),
                                Err(error) => return Err(error),
                            },
                        }
                    } else {
                        match self.find_label(args[1]) {
                            Ok(val) => self.write_word(val),
                            Err(error) => return Err(error),
                        }
                    }
                }
                "NJMP" => {
                    self.write_byte(12);
                    if let Some(imm_type) = Self::get_imm_type(args[1]) {
                        match imm_type {
                            ImmType::Decimal => match self.parse_word_dec(args[1]) {
                                Ok(val) => self.write_word(val),
                                Err(error) => return Err(error),
                            },
                            ImmType::Hex => match self.parse_word_hex(args[1]) {
                                Ok(val) => self.write_word(val),
                                Err(error) => return Err(error),
                            },
                        }
                    } else {
                        match self.find_label(args[1]) {
                            Ok(val) => self.write_word(val),
                            Err(error) => return Err(error),
                        }
                    }
                }
                "CALL" => {
                    self.write_byte(53);
                    if let Some(imm_type) = Self::get_imm_type(args[1]) {
                        match imm_type {
                            ImmType::Decimal => match self.parse_word_dec(args[1]) {
                                Ok(val) => self.write_word(val),
                                Err(error) => return Err(error),
                            },
                            ImmType::Hex => match self.parse_word_hex(args[1]) {
                                Ok(val) => self.write_word(val),
                                Err(error) => return Err(error),
                            },
                        }
                    } else {
                        match self.find_label(args[1]) {
                            Ok(val) => self.write_word(val),
                            Err(error) => return Err(error),
                        }
                    }
                }
                "CCALL" => {
                    self.write_byte(54);
                    if let Some(imm_type) = Self::get_imm_type(args[1]) {
                        match imm_type {
                            ImmType::Decimal => match self.parse_word_dec(args[1]) {
                                Ok(val) => self.write_word(val),
                                Err(error) => return Err(error),
                            },
                            ImmType::Hex => match self.parse_word_hex(args[1]) {
                                Ok(val) => self.write_word(val),
                                Err(error) => return Err(error),
                            },
                        }
                    } else {
                        match self.find_label(args[1]) {
                            Ok(val) => self.write_word(val),
                            Err(error) => return Err(error),
                        }
                    }
                }
                "NCALL" => {
                    self.write_byte(55);
                    if let Some(imm_type) = Self::get_imm_type(args[1]) {
                        match imm_type {
                            ImmType::Decimal => match self.parse_word_dec(args[1]) {
                                Ok(val) => self.write_word(val),
                                Err(error) => return Err(error),
                            },
                            ImmType::Hex => match self.parse_word_hex(args[1]) {
                                Ok(val) => self.write_word(val),
                                Err(error) => return Err(error),
                            },
                        }
                    } else {
                        match self.find_label(args[1]) {
                            Ok(val) => self.write_word(val),
                            Err(error) => return Err(error),
                        }
                    }
                }
                "ADD" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg3, width3) = match self.parse_register(args[3]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num
                        ));
                    }
                    if width2 != width3 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[2], args[3], self.line_num
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(19),
                        Width::Word => self.write_byte(21),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                    self.write_byte(reg3);
                }
                "ADDi" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg3, width3) = match self.parse_register(args[3]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num
                        ));
                    }
                    if width2 != width3 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[2], args[3], self.line_num
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(20),
                        Width::Word => self.write_byte(22),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                    self.write_byte(reg3);
                }
                "SUB" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg3, width3) = match self.parse_register(args[3]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num
                        ));
                    }
                    if width2 != width3 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[2], args[3], self.line_num
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(23),
                        Width::Word => self.write_byte(25),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                    self.write_byte(reg3);
                }
                "SUBi" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg3, width3) = match self.parse_register(args[3]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num
                        ));
                    }
                    if width2 != width3 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[2], args[3], self.line_num
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(24),
                        Width::Word => self.write_byte(26),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                    self.write_byte(reg3);
                }
                "MUL" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg3, width3) = match self.parse_register(args[3]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num
                        ));
                    }
                    if width2 != width3 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[2], args[3], self.line_num
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(27),
                        Width::Word => self.write_byte(29),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                    self.write_byte(reg3);
                }
                "MULi" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg3, width3) = match self.parse_register(args[3]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num
                        ));
                    }
                    if width2 != width3 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[2], args[3], self.line_num
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(28),
                        Width::Word => self.write_byte(30),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                    self.write_byte(reg3);
                }
                "DIV" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg3, width3) = match self.parse_register(args[3]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num
                        ));
                    }
                    if width2 != width3 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[2], args[3], self.line_num
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(31),
                        Width::Word => self.write_byte(33),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                    self.write_byte(reg3);
                }
                "DIVi" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg3, width3) = match self.parse_register(args[3]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num
                        ));
                    }
                    if width2 != width3 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[2], args[3], self.line_num
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(32),
                        Width::Word => self.write_byte(34),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                    self.write_byte(reg3);
                }
                "MOD" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg3, width3) = match self.parse_register(args[3]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num
                        ));
                    }
                    if width2 != width3 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[2], args[3], self.line_num
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(35),
                        Width::Word => self.write_byte(37),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                    self.write_byte(reg3);
                }
                "MODi" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg3, width3) = match self.parse_register(args[3]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num
                        ));
                    }
                    if width2 != width3 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[2], args[3], self.line_num
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(36),
                        Width::Word => self.write_byte(38),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                    self.write_byte(reg3);
                }
                "LES" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(15),
                        Width::Word => self.write_byte(17),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                }
                "LESi" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(16),
                        Width::Word => self.write_byte(18),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                }
                "SHR" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg3, width3) = match self.parse_register(args[3]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num
                        ));
                    }
                    if width2 != width3 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[2], args[3], self.line_num
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(39),
                        Width::Word => self.write_byte(40),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                    self.write_byte(reg3);
                }
                "SHL" => {
                    let (reg1, width1) = match self.parse_register(args[1]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg2, width2) = match self.parse_register(args[2]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };
                    let (reg3, width3) = match self.parse_register(args[3]) {
                        Ok(data) => data,
                        Err(error) => return Err(error),
                    };

                    if width1 != width2 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[1], args[2], self.line_num
                        ));
                    }
                    if width2 != width3 {
                        return Err(format!(
                            "Mismatched register widths {} and {} on line {}",
                            args[2], args[3], self.line_num
                        ));
                    }

                    match width1 {
                        Width::Byte => self.write_byte(41),
                        Width::Word => self.write_byte(42),
                    }

                    self.write_byte(reg1);
                    self.write_byte(reg2);
                    self.write_byte(reg3);
                }
                "DRAW" => self.write_byte(50),
                "RDRAW" => self.write_byte(51),
                "RET" => self.write_byte(48),
                "JDRAW" => {
                    self.write_byte(52);
                    if let Some(imm_type) = Self::get_imm_type(args[1]) {
                        match imm_type {
                            ImmType::Decimal => match self.parse_word_dec(args[1]) {
                                Ok(val) => self.write_word(val),
                                Err(error) => return Err(error),
                            },
                            ImmType::Hex => match self.parse_word_hex(args[1]) {
                                Ok(val) => self.write_word(val),
                                Err(error) => return Err(error),
                            },
                        }
                    } else {
                        match self.find_label(args[1]) {
                            Ok(val) => self.write_word(val),
                            Err(error) => return Err(error),
                        }
                    }
                }
                _ => (),
            }
        }

        Ok(self.program.clone())
    }
}

enum ImmType {
    Decimal,
    Hex,
}
