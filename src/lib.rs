#[macro_use]
extern crate nom;

pub mod bytecode;
pub mod opcodes;
pub mod parser;

pub use bytecode::Instruction;
pub use opcodes::Opcode;
