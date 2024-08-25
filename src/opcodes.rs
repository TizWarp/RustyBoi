#[derive(Debug)]

pub enum Opcode {
    JMP,
    CJMP,
    NJMP,
    MOV { width: OpcodeWidth },
    LOAD { width: OpcodeWidth },
    STORE { width: OpcodeWidth },
    READ { width: OpcodeWidth },
    PUSH { width: OpcodeWidth },
    POP { width: OpcodeWidth },
    EQ { width: OpcodeWidth },
    NEQ { width: OpcodeWidth },
    LES { width: OpcodeWidth, signed: bool },
    ADD { width: OpcodeWidth, signed: bool },
    SUB { width: OpcodeWidth, signed: bool },
    MUL { width: OpcodeWidth, signed: bool },
    DIV { width: OpcodeWidth, signed: bool },
    MOD { width: OpcodeWidth, signed: bool },
    SHR { width: OpcodeWidth },
    SHL { width: OpcodeWidth },
    RET,
    DRAW,
    RDRAW,
    JDRAW,
    CALL,
    CCALL,
    NCALL,
    ILG,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Self::LOAD {
                width: OpcodeWidth::Byte,
            },
            1 => Self::LOAD {
                width: OpcodeWidth::Word,
            },
            2 => Self::MOV {
                width: OpcodeWidth::Byte,
            },
            3 => Self::MOV {
                width: OpcodeWidth::Word,
            },
            4 => Self::STORE {
                width: OpcodeWidth::Byte,
            },
            5 => Self::STORE {
                width: OpcodeWidth::Word,
            },
            6 => Self::READ {
                width: OpcodeWidth::Byte,
            },
            7 => Self::READ {
                width: OpcodeWidth::Word,
            },
            8 => Self::PUSH {
                width: OpcodeWidth::Byte,
            },
            9 => Self::PUSH {
                width: OpcodeWidth::Word,
            },
            10 => Self::JMP,
            11 => Self::CJMP,
            12 => Self::NJMP,
            13 => Self::EQ {
                width: OpcodeWidth::Byte,
            },
            14 => Self::EQ {
                width: OpcodeWidth::Word,
            },

            15 => Self::LES {
                width: OpcodeWidth::Byte,
                signed: false,
            },
            16 => Self::LES {
                width: OpcodeWidth::Byte,
                signed: true,
            },
            17 => Self::LES {
                width: OpcodeWidth::Word,
                signed: false,
            },
            18 => Self::LES {
                width: OpcodeWidth::Word,
                signed: true,
            },

            19 => Self::ADD {
                width: OpcodeWidth::Byte,
                signed: false,
            },
            20 => Self::ADD {
                width: OpcodeWidth::Byte,
                signed: true,
            },
            21 => Self::ADD {
                width: OpcodeWidth::Word,
                signed: false,
            },
            22 => Self::ADD {
                width: OpcodeWidth::Word,
                signed: true,
            },

            23 => Self::SUB {
                width: OpcodeWidth::Byte,
                signed: false,
            },
            24 => Self::SUB {
                width: OpcodeWidth::Byte,
                signed: true,
            },
            25 => Self::SUB {
                width: OpcodeWidth::Word,
                signed: false,
            },
            26 => Self::SUB {
                width: OpcodeWidth::Word,
                signed: true,
            },

            27 => Self::MUL {
                width: OpcodeWidth::Byte,
                signed: false,
            },
            28 => Self::MUL {
                width: OpcodeWidth::Byte,
                signed: true,
            },
            29 => Self::MUL {
                width: OpcodeWidth::Word,
                signed: false,
            },
            30 => Self::MUL {
                width: OpcodeWidth::Word,
                signed: true,
            },

            31 => Self::DIV {
                width: OpcodeWidth::Byte,
                signed: false,
            },
            32 => Self::DIV {
                width: OpcodeWidth::Byte,
                signed: true,
            },
            33 => Self::DIV {
                width: OpcodeWidth::Word,
                signed: false,
            },
            34 => Self::DIV {
                width: OpcodeWidth::Word,
                signed: true,
            },

            35 => Self::MOD {
                width: OpcodeWidth::Byte,
                signed: false,
            },
            36 => Self::MOD {
                width: OpcodeWidth::Byte,
                signed: true,
            },
            37 => Self::MOD {
                width: OpcodeWidth::Word,
                signed: false,
            },
            38 => Self::MOD {
                width: OpcodeWidth::Word,
                signed: true,
            },

            39 => Self::SHR {
                width: OpcodeWidth::Byte,
            },
            40 => Self::SHR {
                width: OpcodeWidth::Word,
            },

            41 => Self::SHL {
                width: OpcodeWidth::Byte,
            },
            42 => Self::SHL {
                width: OpcodeWidth::Word,
            },

            43 => Self::NEQ {
                width: OpcodeWidth::Byte,
            },
            44 => Self::NEQ {
                width: OpcodeWidth::Word,
            },

            45 => Self::POP {
                width: OpcodeWidth::Byte,
            },

            46 => Self::POP {
                width: OpcodeWidth::Word,
            },

            48 => Self::RET,
            50 => Self::DRAW,
            51 => Self::RDRAW,
            52 => Self::JDRAW,
            53 => Self::CALL,
            54 => Self::CCALL,
            55 => Self::NCALL,

            _ => Self::ILG,
        }
    }
}
#[derive(Debug)]
pub enum OpcodeWidth {
    Word,
    Byte,
}
