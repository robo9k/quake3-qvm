/// Size of procedure stack adjustment
pub type FrameSize = u32;

/// Offset within stack frame
pub type FrameOffset = u32;

pub type ArgOffset = u8;

/// Absolute address within code segment
pub type Address = u32;

/// Literal value
pub type Literal = u32;

// These should match their opcodes
#[allow(non_camel_case_types)]
/// A QVM instruction
pub enum Instruction {
    UNDEF,

    IGNORE,

    BREAK,

    ENTER(FrameSize),
    LEAVE(FrameSize),
    CALL,
    PUSH,
    POP,

    CONST(Literal),
    LOCAL(FrameOffset),

    JUMP,

    EQ(Address),
    NE(Address),

    LTI(Address),
    LEI(Address),
    GTI(Address),
    GEI(Address),

    LTU(Address),
    LEU(Address),
    GTU(Address),
    GEU(Address),

    EQF(Address),
    NEF(Address),

    LTF(Address),
    LEF(Address),
    GTF(Address),
    GEF(Address),

    LOAD1,
    LOAD2,
    LOAD4,
    STORE1,
    STORE2,
    STORE4,
    ARG(ArgOffset),

    BLOCK_COPY,

    SEX8,
    SEX16,

    NEGI,
    ADD,
    SUB,
    DIVI,
    DIVU,
    MODI,
    MODU,
    MULI,
    MULU,

    BAND,
    BOR,
    BXOR,
    BCOM,

    LSH,
    RSHI,
    RSHU,

    NEGF,
    ADDF,
    SUBF,
    DIVF,
    MULF,

    CVIF,
    CVFI,
}
