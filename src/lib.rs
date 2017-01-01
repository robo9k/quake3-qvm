// Both `nom` and `error_chain` can recurse deeply
#![recursion_limit="1024"]

#![warn(missing_docs)]

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

use errors::*;

const VM_MAGIC: [u8; 4] = [0x44, 0x14, 0x72, 0x12];

/// A Quake 3 virtual machine image.
///
/// A VM consists of instructions and data, where data is separated into
///
/// * word-sized data
/// * byte-sized data (LIT)
/// * length of uninitialized data (BSS)
#[derive(Debug,PartialEq)]
pub struct QVM {
    code: Vec<Instruction>,
    data: Vec<u32>,
    lit: Vec<u8>,
    bss_length: u32,
}

impl QVM {
    // TODO: Validate instructions; addresses might be out of bounds etc.
    /// Creates a new VM instance.
    ///
    /// # Errors
    /// lorem ipsum
    pub fn new(code: Vec<Instruction>,
               data: Vec<u32>,
               lit: Vec<u8>,
               bss_length: u32)
               -> Result<QVM> {
        Ok(QVM {
            code: code,
            data: data,
            lit: lit,
            bss_length: bss_length,
        })
    }
}
