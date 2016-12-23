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
