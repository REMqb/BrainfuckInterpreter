use super::op_code::OpCode;
use std::io::{self, Read};
use super::state::State;
use std::iter::Iterator;
use super::error::Error;
use crate::bfi::op_code::OpCode::{OpRightBracket, OpLeftBracket};

pub struct BrainFuckInterpreter {
    op_codes: Vec<OpCode>,
    op_code_pointer: usize,
    memory: [u8; 30_000],
    memory_pointer: usize,
    state: State
}

impl BrainFuckInterpreter {
    pub fn new() -> BrainFuckInterpreter {
        BrainFuckInterpreter {
            op_codes: Vec::new(),
            op_code_pointer: 0,
            memory: [0; 30_000],
            memory_pointer: 0,
            state: State::Ready
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        if self.state != State::Ready {
            return Err(Error::NotReady)
        }

        self.state = State::Running;

        loop {
            match self.step() {
                Ok(_) => (),
                Err(e) => match e {
                    Error::LastInstructionReached => {
                        self.state = State::Ended;
                        return Ok(())
                    },
                    _ => {
                        self.state = State::Error;
                        return Err(e)
                    }
                }
            }
        }
    }

    pub fn step(&mut self) -> Result<(), Error> {
        if self.op_codes.len() == 0 {
            return Ok(())
        }

        match self.op_codes[self.op_code_pointer] {
            OpCode::OpPlus(value) => {
                self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_add(value);
            },
            OpCode::OpMinus(value) => {
                self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_sub(value);
            },
            OpCode::OpLeftShift(offset) => {
                self.memory_pointer = self.memory_pointer.wrapping_sub(offset as usize);
                if self.memory_pointer > 29_999 {
                    self.memory_pointer = 29_999
                }
            },
            OpCode::OpRightShift(offset) => {
                self.memory_pointer += offset as usize;
                if self.memory_pointer == 30_000 {
                    self.memory_pointer = 0;
                }
            },
            OpCode::OpLeftBracket(address) => {
                if self.memory[self.memory_pointer] == 0 {
                    match address {
                        Some(jump_to_address) => match self.jump_to(jump_to_address) {
                            Err(_e) => return Err(Error::MissingRightBracket),
                            _ => ()
                        },
                        None => {
                            let mut left_counter = 0;

                            while self.op_code_pointer < self.op_codes.len() {
                                assert!(left_counter >= 0);

                                match self.next_instruction() {
                                    Err(_e) => return Err(Error::MissingRightBracket),
                                    _ => ()
                                }

                                match self.op_codes[self.op_code_pointer] {
                                    OpCode::OpRightBracket(_) => {
                                        if left_counter == 0 {
                                            break;
                                        } else {
                                            left_counter -= 1;
                                        }
                                    },
                                    OpCode::OpLeftBracket(_) => left_counter += 1,
                                    _ => ()
                                }
                            }
                        }
                    }
                }
            },
            OpCode::OpRightBracket(address) => {
                if self.memory[self.memory_pointer] != 0 {
                    match address {
                        Some(jump_to_address) => match self.jump_to(jump_to_address) {
                            Err(_e) => return Err(Error::MissingRightBracket),
                            _ => ()
                        },
                        None => {
                            let mut right_counter = 0;

                            while self.op_code_pointer > 0 {
                                assert!(right_counter >= 0);

                                self.previous_instruction()?;

                                match self.op_codes[self.op_code_pointer] {
                                    OpCode::OpLeftBracket(_) => {
                                        if right_counter == 0 {
                                            break;
                                        } else {
                                            right_counter -= 1;
                                        }
                                    },
                                    OpCode::OpRightBracket(_) => right_counter += 1,
                                    _ => ()
                                }
                            }
                        }
                    }
                }
            },
            OpCode::OpDot => {
                print!("{}", self.memory[self.memory_pointer] as char)
            },
            OpCode::OpComa => {
                let r = io::stdin().bytes().next().and_then(|result| result.ok());

                self.memory[self.memory_pointer] = r.unwrap();
            }
        }
        return self.next_instruction()
    }

    pub fn load(&mut self, data: Vec<u8> ) {
        let mut or_code = OpCode::OpDot;

        for op in data {
            let op_char = op as char;

            match op_char {
                '+' => {
                    match self.op_codes.last_mut().unwrap_or(&mut or_code) {
                        OpCode::OpPlus(value) => *value = value.wrapping_add(1),
                        _ => self.op_codes.push(OpCode::OpPlus(1))
                    }
                },
                '-' => {
                    match self.op_codes.last_mut().unwrap_or(&mut or_code) {
                        OpCode::OpMinus(value) => *value = value.wrapping_add(1),
                        _ => self.op_codes.push(OpCode::OpMinus(1))
                    }
                },
                '<' => {
                    match self.op_codes.last_mut().unwrap_or(&mut or_code) {
                        OpCode::OpLeftShift(value) => *value = value.wrapping_add(1),
                        _ => self.op_codes.push(OpCode::OpLeftShift(1))
                    }
                },
                '>' => {
                    match self.op_codes.last_mut().unwrap_or(&mut or_code) {
                        OpCode::OpRightShift(value) => *value = value.wrapping_add(1),
                        _ => self.op_codes.push(OpCode::OpRightShift(1))
                    }
                },
                '[' => self.op_codes.push(OpCode::OpLeftBracket(None)),
                ']' => self.op_codes.push(OpCode::OpRightBracket(None)),
                '.' => self.op_codes.push(OpCode::OpDot),
                ',' => self.op_codes.push(OpCode::OpComa),
                _ => ()
            }
        }

        self.reset();
    }

    pub fn optimize_jumps(&mut self) {
        let mut new_op_codes = Vec::new();

        for i in 0..self.op_codes.len() {
            match self.op_codes[i] {
                OpCode::OpLeftBracket(address) => {
                    match address {
                        None => {
                            let mut left_counter = 0;
                            let mut instruction_counter = i;

                            while instruction_counter < self.op_codes.len() {
                                assert!(left_counter >= 0);

                                instruction_counter += 1;

                                match self.op_codes[instruction_counter] {
                                    OpCode::OpRightBracket(_) => {
                                        if left_counter == 0 {
                                            break;
                                        } else {
                                            left_counter -= 1;
                                        }
                                    },
                                    OpCode::OpLeftBracket(_) => left_counter += 1,
                                    _ => ()
                                }
                            }

                            new_op_codes.push(OpLeftBracket(Some(instruction_counter)));
                        },
                        _ => ()
                    }
                },
                OpCode::OpRightBracket(address) => {
                    match address {
                        None => {
                            let mut right_counter = 0;
                            let mut instruction_counter = i;

                            while instruction_counter > 0 {
                                assert!(right_counter >= 0);

                                instruction_counter -= 1;

                                match self.op_codes[instruction_counter] {
                                    OpCode::OpLeftBracket(_) => {
                                        if right_counter == 0 {
                                            break;
                                        } else {
                                            right_counter -= 1;
                                        }
                                    },
                                    OpCode::OpRightBracket(_) => right_counter += 1,
                                    _ => ()
                                }
                            }

                            new_op_codes.push(OpRightBracket(Some(instruction_counter)));
                        },
                        _ => ()
                    }
                },
                _ => new_op_codes.push(self.op_codes[i].clone())
            }
        }

        self.op_codes = new_op_codes;
    }

    fn reset_memory(&mut self) {
        if self.state == State::Ready {
            return
        }

        for cell in self.memory.iter_mut() {
            *cell = 0;
        }

        self.memory_pointer = 0
    }

    pub fn reset(&mut self) {
        self.reset_memory();

        self.op_code_pointer = 0;

        self.state = State::Ready;
    }

    fn next_instruction(&mut self) -> Result<(), Error> {
        if self.op_code_pointer == self.op_codes.len() - 1 {
            return Err(Error::LastInstructionReached)
        }
        self.op_code_pointer += 1;
        Ok(())
    }

    fn jump_to(&mut self, address: usize) -> Result<(), Error> {
        if address >= self.op_codes.len() {
            return Err(Error::LastInstructionReached)
        }
        self.op_code_pointer = address;
        Ok(())
    }

    fn previous_instruction(&mut self) -> Result<(), Error>  {
        if self.op_code_pointer == 0 {
            return Err(Error::MissingLeftBracket)
        }
        self.op_code_pointer -= 1;
        Ok(())
    }
}
