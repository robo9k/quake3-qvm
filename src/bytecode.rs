//! Types for the compiled format of a QVM.

/// Size of procedure stack adjustment.
pub type FrameSize = u32;

/// Size of memory block to copy.
pub type BlockSize = u32;

/// Offset within stack frame.
pub type FrameOffset = u32;

/// Offset within the argument marshalling space.
pub type ArgOffset = u8;

/// Absolute instruction offset within code segment.
pub type Address = u32;

/// Literal value.
pub type Literal = u32;

// These should match their opcodes
#[allow(non_camel_case_types)]
/// A QVM instruction.
#[derive(Debug,PartialEq, Copy, Clone)]
pub enum Instruction {
    /// Undefined instruction.
    ///
    /// Used for padding the code segment. Should not occur at runtime.
    UNDEF,

    /// No-operation (NOP).
    IGNORE,

    /// Software breakpoint.
    BREAK,

    /// Enter a procedure, adjusting stack.
    ENTER(FrameSize),
    /// Leave a procedure, adjusting stack.
    LEAVE(FrameSize),
    /// Call a procedure.
    CALL,
    /// Push stack.
    PUSH,
    /// Pop stack.
    POP,

    /// Push constant onto stack.
    CONST(Literal),
    /// Get address of frame local variable or argument.
    LOCAL(FrameOffset),

    /// Jump to top of stack.
    JUMP,

    /// Check (signed integer) equality, jump to `Address` if true.
    EQ(Address),
    /// Check (signed integer) inequality, jump to `Address` if true.
    NE(Address),

    /// Check (signed integer) less-than, jump to `Address` if true.
    LTI(Address),
    /// Check (signed integer) less-than or equal-to, jump to `Address` if true.
    LEI(Address),
    /// Check (signed integer) greater-than, jump to `Address` if true.
    GTI(Address),
    /// Check (signed integer) greater-than or equal-to, jump to `Address` if true.
    GEI(Address),

    /// Check (unsigned integer) less-than, jump to `Address` if true.
    LTU(Address),
    /// Check (unsigned integer) less-than or equal-to, jump to `Address` if true.
    LEU(Address),
    /// Check (unsigned integer) greater-than, jump to `Address` if true.
    GTU(Address),
    /// Check (unsigned integer) greater-than or equal-to, jump to `Address` if true.
    GEU(Address),

    /// Check (float) equality, jump to `Address` if true.
    EQF(Address),
    /// Check (float) inequality, jump to `Address` if true.
    NEF(Address),

    /// Check (float) less-than, jump to `Address` if true.
    LTF(Address),
    /// Check (float) less-than or equal-to, jump to `Address` if true.
    LEF(Address),
    /// Check (float) greater-than, jump to `Address` if true.
    GTF(Address),
    /// Check (float) greater-than or equal-to, jump to `Address` if true.
    GEF(Address),

    /// Load 1-octet value.
    LOAD1,
    /// Load 2-octet value.
    LOAD2,
    /// Load 4-octet value.
    LOAD4,
    /// Store 1-octet value.
    STORE1,
    /// Store 2-octet value.
    STORE2,
    /// Store 4-octet value.
    STORE4,
    /// Store value into marshalling space.
    ARG(ArgOffset),

    /// Copy a block of memory.
    BLOCK_COPY(BlockSize),

    /// Sign-extend 8-bit.
    SEX8,
    /// Sign-extend 16-bit.
    SEX16,

    /// Negate (signed integer).
    NEGI,
    /// Add.
    ADD,
    /// Subtract.
    SUB,
    /// Divide (signed integer).
    DIVI,
    /// Divide (unsigned integer).
    DIVU,
    /// Modulo (signed integer).
    MODI,
    /// Modulo (unsigned integer).
    MODU,
    /// Multiply (signed integer).
    MULI,
    /// Multiply (unsigned integer).
    MULU,

    /// Bitwise AND.
    BAND,
    /// Bitwise OR.
    BOR,
    /// Bitwise XOR.
    BXOR,
    /// Bitwise complement.
    BCOM,

    /// Bitwise left-shift.
    LSH,
    /// Algebraic (signed) right-shift.
    RSHI,
    /// Bitwise (unsigned) right-shift.
    RSHU,

    /// Negate (float).
    NEGF,
    /// Add (float).
    ADDF,
    /// Subtract (float).
    SUBF,
    /// Divide (float).
    DIVF,
    /// Multiply (float).
    MULF,

    /// Convert signed integer to float.
    CVIF,
    /// Convert float to signed integer.
    CVFI,
}
