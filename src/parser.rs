use super::{opcodes, bytecode};
use nom::*;

type Input = u8;
type InputSlice<'a> = &'a [Input];

named!(op_break<InputSlice,bytecode::Instruction>,
    value!(
        bytecode::Instruction::BREAK,
        tag!([opcodes::Opcode::BREAK as Input])
    )
);

named!(op_enter<InputSlice,bytecode::Instruction>,
    do_parse!(
        tag!([opcodes::Opcode::ENTER as Input]) >>
        frame_size: le_u32             >>
        (bytecode::Instruction::ENTER(frame_size))
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_break_exact_match() {
        let data = [0x2];
        let result = op_break(&data);
        assert_eq!(result, IResult::Done(&b""[..], bytecode::Instruction::BREAK));
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
        assert_eq!(result, IResult::Done(&b""[..], bytecode::Instruction::ENTER(0x42)));
    }

}
