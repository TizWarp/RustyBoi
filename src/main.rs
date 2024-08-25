use std::{
    env::{self, consts::OS},
    fs::File,
    io::{Read, Write},
};

use assembler::Assembler;
//use assembler::Assembler;
use rusty_boi::RustyBoi;
use vm_main::run_vm;

pub mod assembler;
pub mod compare;
pub mod math;
pub mod opcodes;
pub mod register_file;
pub mod rusty_boi;
pub mod vm_main;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "c" => {
            let mut assembler = Assembler::new();
            assembler.load_file(args[2].as_str())?;
            assembler.pre_assembly()?;
            let mut program = assembler.assemble()?;
            let mut out_file = File::create(args[3].clone()).map_err(|e| e.to_string())?;
            out_file.write(&mut program).map_err(|e| e.to_string())?;
        }
        "e" => {
            let mut program_buffer: Vec<u8> = Vec::new();
            let mut file = File::open(args[2].clone()).map_err(|e| e.to_string())?;
            file.read_to_end(&mut program_buffer)
                .map_err(|e| e.to_string())?;
            let mut rusty_boi = RustyBoi::new();
            rusty_boi.load_program(program_buffer);
            run_vm(rusty_boi)?;
        }
        _ => return Err(format!("Uknown operation {}", args[1])),
    }

    Ok(())
}
