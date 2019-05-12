use crate::bfi::op_code::OpCode::*;

pub enum OpCode {
    OpPlus(u8),
    OpMinus(u8),
    OpDot,
    OpComa,
    OpLeftShift(u16),
    OpRightShift(u16),
    OpLeftBracket(Option<usize>),
    OpRightBracket(Option<usize>)
}

impl Clone for OpCode {
    fn clone(&self) -> Self {
        match self {
            OpPlus(value) => OpPlus(*value),
            OpMinus(value) => OpMinus(*value),
            OpDot => OpDot,
            OpComa => OpComa,
            OpLeftShift(value) => OpLeftShift(*value),
            OpRightShift(value) => OpRightShift(*value),
            OpLeftBracket(value) => OpLeftBracket(*value),
            OpRightBracket(value) => OpRightBracket(*value)
        }
    }

    fn clone_from(&mut self, _source: &Self) {
        unimplemented!()
    }
}