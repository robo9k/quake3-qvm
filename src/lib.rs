#![recursion_limit="128"]

#[macro_use]
extern crate nom;

pub mod bytecode;
pub mod opcodes;
pub mod parser;

pub use bytecode::Instruction;
pub use opcodes::Opcode;

const VM_MAGIC: [u8; 4] = [0x44, 0x14, 0x72, 0x12];

#[derive(Debug,PartialEq)]
struct QVM {
    code: Vec<Instruction>,
    data: Vec<u32>,
    lit: Vec<u8>,
    bss_length: u32,
}
