// Both `nom` and `error_chain` can recurse deeply
#![recursion_limit="1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate nom;

pub mod errors;
pub mod bytecode;
pub mod opcodes;
pub mod parser;

pub use bytecode::Instruction;
pub use opcodes::Opcode;

const VM_MAGIC: [u8; 4] = [0x44, 0x14, 0x72, 0x12];

// TODO: Validate instructions in new(), since Addresses might be out of bounds etc.
#[derive(Debug,PartialEq)]
struct QVM {
    code: Vec<Instruction>,
    data: Vec<u32>,
    lit: Vec<u8>,
    bss_length: u32,
}
