use super::{Opcode, Instruction, QVM, VM_MAGIC};
use super::errors::*;
use nom;
use nom::{le_u32, le_u8};

type Input = u8;
type InputSlice<'a> = &'a [Input];

/// Creates a named parser for an instruction that only consists of an opcode
macro_rules! instruction {
    ($name:ident, $opcode:path, $instruction:path) => {
        named!($name<InputSlice,Instruction>,
            value!($instruction, tag!([$opcode as Input]))
        );
    }
}

/// Creates a named parser for an instruction that has an operand
macro_rules! instruction_with_operand {
    ($name:ident, $opcode:path, $operand_fn:ident, $instruction:path) => {
        named!($name<InputSlice,Instruction>,
            do_parse!(
                tag!([$opcode as Input]) >>
                operand: $operand_fn      >>
                ($instruction(operand))
            )
        );
    }
}

/// Creates a named parser for an instruction with `u32` operand
macro_rules! instruction_u32 {
    ($name:ident, $opcode:path, $instruction:path) => {
        instruction_with_operand!($name, $opcode, le_u32, $instruction);
    }
}

/// Creates a named parser for an instruction with `u8` operand
macro_rules! instruction_u8 {
    ($name:ident, $opcode:path, $instruction:path) => {
        instruction_with_operand!($name, $opcode, le_u8, $instruction);
    }
}


instruction!(instruction_undef, Opcode::UNDEF, Instruction::UNDEF);

instruction!(instruction_ignore, Opcode::IGNORE, Instruction::IGNORE);

instruction!(instruction_break, Opcode::BREAK, Instruction::BREAK);

instruction_u32!(instruction_enter, Opcode::ENTER, Instruction::ENTER);
instruction_u32!(instruction_leave, Opcode::LEAVE, Instruction::LEAVE);
instruction!(instruction_call, Opcode::CALL, Instruction::CALL);
instruction!(instruction_push, Opcode::PUSH, Instruction::PUSH);
instruction!(instruction_pop, Opcode::POP, Instruction::POP);

instruction_u32!(instruction_const, Opcode::CONST, Instruction::CONST);
instruction_u32!(instruction_local, Opcode::LOCAL, Instruction::LOCAL);

instruction!(instruction_jump, Opcode::JUMP, Instruction::JUMP);

instruction_u32!(instruction_eq, Opcode::EQ, Instruction::EQ);
instruction_u32!(instruction_ne, Opcode::NE, Instruction::NE);

instruction_u32!(instruction_lti, Opcode::LTI, Instruction::LTI);
instruction_u32!(instruction_lei, Opcode::LEI, Instruction::LEI);
instruction_u32!(instruction_gti, Opcode::GTI, Instruction::GTI);
instruction_u32!(instruction_gei, Opcode::GEI, Instruction::GEI);

instruction_u32!(instruction_ltu, Opcode::LTU, Instruction::LTU);
instruction_u32!(instruction_leu, Opcode::LEU, Instruction::LEU);
instruction_u32!(instruction_gtu, Opcode::GTU, Instruction::GTU);
instruction_u32!(instruction_geu, Opcode::GEU, Instruction::GEU);

instruction_u32!(instruction_eqf, Opcode::EQF, Instruction::EQF);
instruction_u32!(instruction_nef, Opcode::NEF, Instruction::NEF);

instruction_u32!(instruction_ltf, Opcode::LTF, Instruction::LTF);
instruction_u32!(instruction_lef, Opcode::LEF, Instruction::LEF);
instruction_u32!(instruction_gtf, Opcode::GTF, Instruction::GTF);
instruction_u32!(instruction_gef, Opcode::GEF, Instruction::GEF);

instruction!(instruction_load1, Opcode::LOAD1, Instruction::LOAD1);
instruction!(instruction_load2, Opcode::LOAD2, Instruction::LOAD2);
instruction!(instruction_load4, Opcode::LOAD4, Instruction::LOAD4);
instruction!(instruction_store1, Opcode::STORE1, Instruction::STORE1);
instruction!(instruction_store2, Opcode::STORE2, Instruction::STORE2);
instruction!(instruction_store4, Opcode::STORE4, Instruction::STORE4);
instruction_u8!(instruction_arg, Opcode::ARG, Instruction::ARG);

instruction_u32!(instruction_block_copy,
                 Opcode::BLOCK_COPY,
                 Instruction::BLOCK_COPY);

instruction!(instruction_sex8, Opcode::SEX8, Instruction::SEX8);
instruction!(instruction_sex16, Opcode::SEX16, Instruction::SEX16);

instruction!(instruction_negi, Opcode::NEGI, Instruction::NEGI);
instruction!(instruction_add, Opcode::ADD, Instruction::ADD);
instruction!(instruction_sub, Opcode::SUB, Instruction::SUB);
instruction!(instruction_divi, Opcode::DIVI, Instruction::DIVI);
instruction!(instruction_divu, Opcode::DIVU, Instruction::DIVU);
instruction!(instruction_modi, Opcode::MODI, Instruction::MODI);
instruction!(instruction_modu, Opcode::MODU, Instruction::MODU);
instruction!(instruction_muli, Opcode::MULI, Instruction::MULI);
instruction!(instruction_mulu, Opcode::MULU, Instruction::MULU);

instruction!(instruction_band, Opcode::BAND, Instruction::BAND);
instruction!(instruction_bor, Opcode::BOR, Instruction::BOR);
instruction!(instruction_bxor, Opcode::BXOR, Instruction::BXOR);
instruction!(instruction_bcom, Opcode::BCOM, Instruction::BCOM);

instruction!(instruction_lsh, Opcode::LSH, Instruction::LSH);
instruction!(instruction_rshi, Opcode::RSHI, Instruction::RSHI);
instruction!(instruction_rshu, Opcode::RSHU, Instruction::RSHU);

instruction!(instruction_negf, Opcode::NEGF, Instruction::NEGF);
instruction!(instruction_addf, Opcode::ADDF, Instruction::ADDF);
instruction!(instruction_subf, Opcode::SUBF, Instruction::SUBF);
instruction!(instruction_divf, Opcode::DIVF, Instruction::DIVF);
instruction!(instruction_mulf, Opcode::MULF, Instruction::MULF);

instruction!(instruction_cvif, Opcode::CVIF, Instruction::CVIF);
instruction!(instruction_cvfi, Opcode::CVFI, Instruction::CVFI);


named!(ins<InputSlice,Instruction>,
    alt!(instruction_undef
        | instruction_ignore
        | instruction_break
        | instruction_enter | instruction_leave | instruction_call | instruction_push | instruction_pop
        | instruction_const | instruction_local
        | instruction_jump
        | instruction_eq | instruction_ne
        | instruction_lti | instruction_lei | instruction_gti | instruction_gei
        | instruction_ltu | instruction_leu | instruction_gtu | instruction_geu
        | instruction_eqf | instruction_nef
        | instruction_ltf | instruction_lef | instruction_gtf | instruction_gef
        | instruction_load1 | instruction_load2 | instruction_load4 | instruction_store1 | instruction_store2 | instruction_store4 | instruction_arg
        | instruction_block_copy
        | instruction_sex8 | instruction_sex16
        | instruction_negi | instruction_add | instruction_sub | instruction_divi | instruction_divu | instruction_modi | instruction_modu | instruction_muli | instruction_mulu
        | instruction_band | instruction_bor | instruction_bxor | instruction_bcom
        | instruction_lsh  | instruction_rshi | instruction_rshu
        | instruction_negf | instruction_addf | instruction_subf | instruction_divf | instruction_mulf
        | instruction_cvif | instruction_cvfi
    )
);

macro_rules! length_size(
    ($i:expr, $s:expr, $submac:ident!( $($args:tt)* )) => (
    {
        match take!($i, $s as usize) {
            nom::IResult::Error(e)                         => nom::IResult::Error(e),
            nom::IResult::Incomplete(nom::Needed::Unknown) => nom::IResult::Incomplete(nom::Needed::Unknown),
            nom::IResult::Incomplete(nom::Needed::Size(n)) => {
                nom::IResult::Incomplete(nom::Needed::Size(
                    n + nom::InputLength::input_len(&($i)) - ($s)
                ))
            },
            nom::IResult::Done(i2, o2)  => {
                match complete!(o2, $submac!($($args)*)) {
                    nom::IResult::Error(e)      => nom::IResult::Error(e),
                    nom::IResult::Incomplete(i) => nom::IResult::Incomplete(i),
                    nom::IResult::Done(_, o3)   => nom::IResult::Done(i2, o3)
                }
            }
        }
    }
    );

  ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
    length_value!($i, $submac!($($args)*), call!($g));
  );

  ($i:expr, $f:expr, $submac:ident!( $($args:tt)* )) => (
    length_value!($i, call!($f), $submac!($($args)*));
  );

  ($i:expr, $f:expr, $g:expr) => (
    length_value!($i, call!($f), call!($g));
  );
);

const HEADER_LENGTH_V1: u32 = 32;

named!(qvm<InputSlice, QVM>,
    do_parse!(
        tag!(VM_MAGIC)                                  >>
//        magic: le_u32                                   >>
        instruction_count: le_u32                       >>
        code_offset: le_u32                             >>
        code_length: le_u32                             >>
        data_offset: le_u32                             >>
        data_length: le_u32                             >>
        lit_length: le_u32                              >>
        bss_length: le_u32                              >>
        // Read padding between header and code segment
        take!(code_offset - HEADER_LENGTH_V1)           >>
        code: length_size!(
            code_length as usize,
            count!(ins, instruction_count as usize)
        )                                               >>
        // Read padding between code and data segment
        take!(data_offset - code_offset - code_length)  >>
        data: length_size!(
            data_length as usize,
            count!(le_u32, data_length as usize / 4)
        )                                               >>
        // lit segment is always aligned, no padding here
        lit: length_size!(
            lit_length as usize,
            count!(le_u8, lit_length as usize)
        )                                               >>
        eof!()                                          >>
        (
            QVM {
                code: code,
                data: data,
                lit: lit,
                bss_length: bss_length,
            }
        )
    )
);


/// Tries to parse a QVM from a byte slice.
pub fn parse_qvm(data: InputSlice) -> Result<QVM> {
    match qvm(data).to_full_result() {
        Ok(v) => Ok(v),
        Err(e) => Err(ErrorKind::Parser(e).into()),
    }
}


#[cfg(test)]
mod tests {
    use super::{instruction_break, instruction_enter, instruction_arg, ins, qvm, parse_qvm,
                InputSlice};
    use bytecode::Instruction;
    use nom::IResult;
    use nom;
    use ::QVM;

    /// q3asm reserves the stack in the BSS segment
    const Q3ASM_STACK_SIZE: usize = 0x10000;

    #[test]
    fn test_instruction_break_exact_match() {
        let data = [0x2];
        let result = instruction_break(&data);
        assert_eq!(result, IResult::Done(&b""[..], Instruction::BREAK));
    }

    #[test]
    fn test_instruction_break_tag_mismatch() {
        let data = [0x0];
        let result = instruction_break(&data);
        assert_eq!(result, IResult::Error(nom::ErrorKind::Tag));
    }

    #[test]
    fn test_instruction_enter_exact_match() {
        let data = [0x3, 0x42, 0x0, 0x0, 0x0];
        let result = instruction_enter(&data);
        assert_eq!(result, IResult::Done(&b""[..], Instruction::ENTER(0x42)));
    }

    #[test]
    fn test_instruction_arg_exact_match() {
        let data = [0x21, 0x42];
        let result = instruction_arg(&data);
        assert_eq!(result, IResult::Done(&b""[..], Instruction::ARG(0x42)));
    }

    #[test]
    fn test_ins_enter_exact_match() {
        let data = [0x3, 0x42, 0x0, 0x0, 0x0];
        let result = ins(&data);
        assert_eq!(result, IResult::Done(&b""[..], Instruction::ENTER(0x42)));
    }

    #[test]
    fn test_qvm_file_minimal() {
        let data = include_bytes!("../assets/mod-minimal.qvm");
        let result = qvm(data);
        let expected = QVM {
            code: vec![
                Instruction::ENTER(8),
                Instruction::CONST(4294967295), // TODO: This is actually -1, need to rethink types!
                Instruction::LEAVE(8),
                Instruction::PUSH,
                Instruction::LEAVE(8),
            ],
            data: vec![
                0
            ],
            lit: vec![],
            bss_length: Q3ASM_STACK_SIZE as u32,
        };
        assert_eq!(result, IResult::Done(&b""[..], expected));
    }

    #[test]
    fn test_qvm_file_bss() {
        let data = include_bytes!("../assets/mod-bss.qvm");
        let result = qvm(data);
        let expected = QVM {
            code: vec![
                Instruction::ENTER(8),
                Instruction::CONST(4294967295), // TODO: This is actually -1, need to rethink types!
                Instruction::LEAVE(8),
                Instruction::PUSH,
                Instruction::LEAVE(8),
            ],
            data: vec![
                0
            ],
            lit: vec![],
            bss_length: Q3ASM_STACK_SIZE as u32 + 4,
        };
        assert_eq!(result, IResult::Done(&b""[..], expected));
    }

    #[test]
    fn test_qvm_file_data() {
        let data = include_bytes!("../assets/mod-data.qvm");
        let result = qvm(data);
        let expected = QVM {
            code: vec![
                Instruction::ENTER(8),
                Instruction::CONST(4294967295), // TODO: This is actually -1, need to rethink types!
                Instruction::LEAVE(8),
                Instruction::PUSH,
                Instruction::LEAVE(8),
            ],
            data: vec![
                0, // for alignment?
                0xDEADBEEF,
            ],
            lit: vec![],
            bss_length: Q3ASM_STACK_SIZE as u32,
        };
        assert_eq!(result, IResult::Done(&b""[..], expected));
    }

    #[test]
    fn test_qvm_file_lit() {
        let data = include_bytes!("../assets/mod-lit.qvm");
        let result = qvm(data);
        let expected = QVM {
            code: vec![
                Instruction::ENTER(8),
                Instruction::CONST(4294967295), // TODO: This is actually -1, need to rethink types!
                Instruction::LEAVE(8),
                Instruction::PUSH,
                Instruction::LEAVE(8),
            ],
            data: vec![
                0,
            ],
            lit: vec![
                '!' as u8,
                0, // padding for aligment?
                0,
                0,
            ],
            bss_length: Q3ASM_STACK_SIZE as u32,
        };
        assert_eq!(result, IResult::Done(&b""[..], expected));
    }


    #[test]
    fn test_ins_file() {
        let data = include_bytes!("../assets/mod-minimal.qvm");
        named!(ins5<InputSlice,Vec<Instruction>>, count!(ins, 5));
        let result = ins5(&data[32..53]);
        let expected = vec![
            Instruction::ENTER(8),
            Instruction::CONST(4294967295), // TODO: This is actually -1, need to rethink types!
            Instruction::LEAVE(8),
            Instruction::PUSH,
            Instruction::LEAVE(8),
        ];
        assert_eq!(result, IResult::Done(&b""[..], expected));
    }

    // TODO: This is more of an integration test
    #[test]
    // TODO: This test won't work due to v2 magic, which is unimplemented
    #[ignore]
    fn test_parse_qvm_ioq3_qagame() {
        let data = include_bytes!("../assets/ioq3/baseq3/vm/qagame.qvm");
        let result = parse_qvm(data).unwrap();
        // TODO: What to assert here?
    }

}
