use core::panic;
use std::array;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(PartialEq, Clone, Debug)]
enum Op {
    OpEnd,
    OpIncDp,
    OpDecDp,
    OpIncVal,
    OpDecVal,
    OpOut,
    OpIn,
    OpJmpFwd,
    OpJmpBck,
}

enum Statuses {
    Success,
    Failure,
}

const PROGRAM_SIZE: u16 = 4096;
const STACK_SIZE: usize = 512;
const DATA_SIZE: usize = 65535;

#[derive(Debug)]
enum StackErrors {
    OverFlow,
    UnderFlow,
}

struct Stack {
    ptr: usize,
    arr: [u16; STACK_SIZE],
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            ptr: 0,
            arr: [0; STACK_SIZE],
        }
    }

    pub fn push(&mut self, a: u16) -> Result<(), StackErrors> {
        if self.ptr >= STACK_SIZE {
            return Err(StackErrors::OverFlow);
        }

        self.arr[self.ptr] = a;
        self.ptr += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<u16, StackErrors> {
        if self.ptr == 0 {
            return Err(StackErrors::UnderFlow);
        }

        self.ptr -= 1;

        Ok(self.arr[self.ptr])
    }

    pub fn is_empty(&self) -> bool {
        match self.ptr {
            0 => true,
            _ => false,
        }
    }

    pub fn is_full(&self) -> bool {
        match self.ptr {
            STACK_SIZE => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
struct Instruction {
    pub operator: Op,
    pub operand: u16,
}

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            operator: Op::OpEnd,
            operand: 0,
        }
    }
}

struct Program {
    instructions: [Instruction; PROGRAM_SIZE as usize],
    stack: Stack,
}

impl Program {
    pub fn new() -> Program {
        Program {
            instructions: array::from_fn(|_| Instruction::default()),
            stack: Stack::new(),
        }
    }

    pub fn compile(&mut self, fp: &String) -> Statuses {
        let mut pc = 0;

        for c in fp.trim().chars() {
            if !pc < PROGRAM_SIZE {
                break;
            }

            let idx = pc as usize;

            match c {
                '>' => self.instructions[idx].operator = Op::OpIncDp,
                '<' => self.instructions[idx].operator = Op::OpDecDp,
                '+' => self.instructions[idx].operator = Op::OpIncVal,
                '-' => self.instructions[idx].operator = Op::OpDecVal,
                '.' => self.instructions[idx].operator = Op::OpOut,
                ',' => self.instructions[idx].operator = Op::OpIn,
                '[' => {
                    self.instructions[idx].operator = Op::OpJmpFwd;

                    if self.stack.is_full() {
                        return Statuses::Failure;
                    }

                    self.stack
                        .push(pc)
                        .expect("Critical error, failed to push to stack");
                }
                ']' => {
                    if self.stack.is_empty() {
                        return Statuses::Failure;
                    }

                    let jmp_pc = self
                        .stack
                        .pop()
                        .expect("Critical error, failed to pop value off stack");

                    self.instructions[idx].operator = Op::OpJmpBck;
                    self.instructions[idx].operand = jmp_pc;
                    self.instructions[jmp_pc as usize].operand = jmp_pc;
                }
                _ => pc = pc.wrapping_sub(1),
            }

            pc = pc.wrapping_add(1);
        }

        if !self.stack.is_empty() || pc == PROGRAM_SIZE {
            return Statuses::Failure;
        }

        self.instructions[pc as usize].operator = Op::OpEnd;

        Statuses::Success
    }

    pub fn execute(&mut self) -> Statuses {
        let mut data = [0; DATA_SIZE];
        let mut pc = 0;
        let mut ptr = 0;

        while (self.instructions[pc].operator != Op::OpEnd) && (ptr < DATA_SIZE) {
            match self.instructions[pc].operator {
                Op::OpIncDp => ptr += 1,
                Op::OpDecDp => ptr -= 1,
                Op::OpIncVal => data[ptr] += 1,
                Op::OpDecVal => data[ptr] -= 1,
                Op::OpOut => print!(
                    "{}",
                    char::from_u32(data[ptr]).expect("failed to convert data to char")
                ),
                Op::OpIn => {
                    data[ptr] = {
                        let mut buffer = [0u8; 2];
                        match io::stdin().read_exact(&mut buffer) {
                            Ok(_) => u16::from_be_bytes(buffer).into(),
                            Err(_) => panic!("Failed to convert input to u16 char"),
                        }
                    }
                }
                Op::OpJmpFwd => {
                    if data[ptr] == 0 {
                        pc = self.instructions[pc].operand as usize
                    }
                }
                Op::OpJmpBck => {
                    if data[ptr] != 0 {
                        pc = self.instructions[pc].operand as usize
                    }
                }
                _ => return Statuses::Failure,
            }
            pc += 1;
        }

        match ptr != DATA_SIZE {
            true => Statuses::Success,
            false => Statuses::Failure,
        }
    }
}

#[derive(Debug)]
enum Error {
    FailedToExecute,
    FailedToCompile,
}

fn main() -> Result<(), Error> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2
        || !Path::new(
            args.get(1)
                .expect("The compiler could not find a file argument"),
        )
        .exists()
    {
        eprintln!("Usage: {} filename\n", args[0]);
    }

    let mut buffer = String::new();
    let mut file = File::open(&args[1]).expect("Could not open the file given");
    file.read_to_string(&mut buffer)
        .expect("Coud not read the file given");

    let mut prog = Program::new();

    match prog.compile(&buffer) {
        Statuses::Success => match prog.execute() {
            Statuses::Success => Ok(()),
            Statuses::Failure => Err(Error::FailedToExecute),
        },
        Statuses::Failure => Err(Error::FailedToCompile),
    }
}
