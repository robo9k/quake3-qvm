use super::{Opcode, Instruction};
use nom::*;

type Input = u8;
type InputSlice<'a> = &'a [Input];

macro_rules! op {
    ($name:ident, $op:path, $ins:path) => {
        named!($name<InputSlice,Instruction>,
            value!($ins, tag!([$op as Input])
            )
        );
    }
}

macro_rules! op_fn {
    ($name:ident, $op:path, $op_fn:ident, $ins:path) => {
        named!($name<InputSlice,Instruction>,
            do_parse!(
                tag!([$op as Input]) >>
                operand: $op_fn      >>
                ($ins(operand))
            )
        );
    }
}

macro_rules! op_u32 {
    ($name:ident, $op:path, $ins:path) => {
        op_fn!($name, $op, le_u32, $ins);
    }
}


op!(op_undef, Opcode::UNDEF, Instruction::UNDEF);

op!(op_ignore, Opcode::IGNORE, Instruction::IGNORE);

op!(op_break, Opcode::BREAK, Instruction::BREAK);

op_u32!(op_enter, Opcode::ENTER, Instruction::ENTER);
op_u32!(op_leave, Opcode::LEAVE, Instruction::LEAVE);
op!(op_call, Opcode::CALL, Instruction::CALL);
op!(op_push, Opcode::PUSH, Instruction::PUSH);
op!(op_pop, Opcode::POP, Instruction::POP);

op_u32!(op_const, Opcode::CONST, Instruction::CONST);
op_u32!(op_local, Opcode::LOCAL, Instruction::LOCAL);

op!(op_jump, Opcode::JUMP, Instruction::JUMP);

op_u32!(op_eq, Opcode::EQ, Instruction::EQ);
op_u32!(op_ne, Opcode::NE, Instruction::NE);

op_u32!(op_lti, Opcode::LTI, Instruction::LTI);
op_u32!(op_lei, Opcode::LEI, Instruction::LEI);
op_u32!(op_gti, Opcode::GTI, Instruction::GTI);
op_u32!(op_gei, Opcode::GEI, Instruction::GEI);

op_u32!(op_ltu, Opcode::LTU, Instruction::LTU);
op_u32!(op_leu, Opcode::LEU, Instruction::LEU);
op_u32!(op_gtu, Opcode::GTU, Instruction::GTU);
op_u32!(op_geu, Opcode::GEU, Instruction::GEU);

op_u32!(op_eqf, Opcode::EQF, Instruction::EQF);
op_u32!(op_nef, Opcode::NEF, Instruction::NEF);

op_u32!(op_ltf, Opcode::LTF, Instruction::LTF);
op_u32!(op_lef, Opcode::LEF, Instruction::LEF);
op_u32!(op_gtf, Opcode::GTF, Instruction::GTF);
op_u32!(op_gef, Opcode::GEF, Instruction::GEF);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_break_exact_match() {
        let data = [0x2];
        let result = op_break(&data);
        assert_eq!(result, IResult::Done(&b""[..], Instruction::BREAK));
    }

    #[test]
    fn test_op_break_tag_mismatch() {
        let data = [0x0];
        let result = op_break(&data);
        assert_eq!(result, IResult::Error(ErrorKind::Tag));
    }

    #[test]
    fn test_op_enter_exact_match() {
        let data = [0x3, 0x42, 0x0, 0x0, 0x0];
        let result = op_enter(&data);
        assert_eq!(result, IResult::Done(&b""[..], Instruction::ENTER(0x42)));
    }

}
