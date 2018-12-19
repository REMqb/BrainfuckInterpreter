use super::op_code::OpCode;
use std::io::{self, Read};
use super::state::State;
use std::iter::Iterator;
use super::error::Error;

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
            OpCode::OpPlus => {
                self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_add(1);
            },
            OpCode::OpMinus => {
                self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_sub(1);
            },
            OpCode::OpLeftShift => {
                self.memory_pointer = self.memory_pointer.wrapping_sub(1);
                if self.memory_pointer > 29_999 {
                    self.memory_pointer = 29_999
                }
            },
            OpCode::OpRightShift => {
                self.memory_pointer += 1;
                if self.memory_pointer == 30_000 {
                    self.memory_pointer = 0;
                }
            },
            OpCode::OpLeftBracket => {
                if self.memory[self.memory_pointer] == 0 {
                    let mut left_counter = 0;

                    while self.op_code_pointer < self.op_codes.len() {
                        assert!(left_counter >= 0);

                        match self.next_instruction() {
                            Err(_e) => return Err(Error::MissingRightBracket),
                            _ => ()
                        }

                        match self.op_codes[self.op_code_pointer] {
                            OpCode::OpRightBracket => {
                                if left_counter == 0 {
                                    break;
                                } else {
                                    left_counter -= 1;
                                }
                            },
                            OpCode::OpLeftBracket => left_counter += 1,
                            _ => ()
                        }
                    }
                }
            },
            OpCode::OpRightBracket => {
                if self.memory[self.memory_pointer] != 0 {
                    let mut right_counter = 0;

                    while self.op_code_pointer < self.op_codes.len() {
                        assert!(right_counter >= 0);

                        self.previous_instruction()?;

                        match self.op_codes[self.op_code_pointer] {
                            OpCode::OpLeftBracket => {
                                if right_counter == 0 {
                                    break;
                                } else {
                                    right_counter -= 1;
                                }
                            },
                            OpCode::OpRightBracket => right_counter += 1,
                            _ => ()
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
        for op in data {
            let op_char = op as char;

            match op_char {
                '+' => self.op_codes.push(OpCode::OpPlus),
                '-' => self.op_codes.push(OpCode::OpMinus),
                '<' => self.op_codes.push(OpCode::OpLeftShift),
                '>' => self.op_codes.push(OpCode::OpRightShift),
                '[' => self.op_codes.push(OpCode::OpLeftBracket),
                ']' => self.op_codes.push(OpCode::OpRightBracket),
                '.' => self.op_codes.push(OpCode::OpDot),
                ',' => self.op_codes.push(OpCode::OpComa),
                _ => ()
            }
        }

        self.reset();
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

    fn previous_instruction(&mut self) -> Result<(), Error>  {
        if self.op_code_pointer == 0 {
            return Err(Error::MissingLeftBracket)
        }
        self.op_code_pointer -= 1;
        Ok(())
    }
}
