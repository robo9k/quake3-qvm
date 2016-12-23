#![recursion_limit="128"]

#[macro_use]
extern crate nom;

pub mod bytecode;
pub mod opcodes;
pub mod parser;

pub use bytecode::Instruction;
pub use opcodes::Opcode;
