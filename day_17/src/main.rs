use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines, Write},
    iter,
};

const FILE_LOC: &'static str = "data/input.txt";

fn main() {
    {
        let (mut registers, program, string_program) = read_file();
        let mut instruction_pointer = 0;
        let mut output_string = String::with_capacity(20);

        while instruction_pointer < program.len() as i64 {
            registers.apply_operation(
                &program[instruction_pointer as usize],
                &mut instruction_pointer,
                &mut output_string,
            );
        }

        println!("Solution: {}", output_string.strip_suffix(",").unwrap())
    }

    println!("Part 1 complete.");

    // Example is simply (3*8**5 + 4*8**4  + 5*8**3 + 3*8**2) = 117440
    {
        let (original_registers, program, string_program) = read_file();
        let mut start_output_string = String::with_capacity(20);
        let mut iter_counter = 1649267441664;
        while string_program != start_output_string.trim_end_matches(",") {
            let mut this_iter_registers = Registers {
                a: iter_counter,
                b: 0,
                c: 0,
            };
            let mut instruction_pointer = 0;
            let mut output_string = String::with_capacity(20);

            while instruction_pointer < program.len() as i64 {
                this_iter_registers.apply_operation(
                    &program[instruction_pointer as usize],
                    &mut instruction_pointer,
                    &mut output_string,
                );
            }

            start_output_string = output_string;

            if iter_counter % 1 == 0 {
                println!(
                    "Target: {}, solution: {}, counter: {}",
                    string_program,
                    start_output_string.trim_end_matches(","),
                    iter_counter,
                );
            } //7562301
            iter_counter += 1;
        }
        println!("Final a register value: {}", iter_counter - 8);
    }
}

fn read_file() -> (Registers, Vec<ProgramIteration>, String) {
    let file = File::open(FILE_LOC).unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let register = Registers {
        a: get_reg_val(&mut lines),
        b: get_reg_val(&mut lines),
        c: get_reg_val(&mut lines),
    };
    lines.next();

    let program = get_program(&mut lines);
    return (register, program.0, program.1);
}

fn get_reg_val(lines: &mut Lines<BufReader<File>>) -> i64 {
    lines
        .next()
        .unwrap()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .parse::<i64>()
        .unwrap()
}

fn get_program(lines: &mut Lines<BufReader<File>>) -> (Vec<ProgramIteration>, String) {
    let string_program = lines
        .next()
        .unwrap()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .to_owned();
    let prog = string_program
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
        .chunks(2)
        .map(|x| ProgramIteration {
            operand: x[1].into(),
            opcode: x[0].into(),
        })
        .collect();

    return (prog, string_program);
}

#[derive(Debug)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

impl Registers {
    fn apply_operation(
        &mut self,
        op: &ProgramIteration,
        instruction_pointer: &mut i64,
        output_string: &mut String,
    ) {
        match op.opcode {
            OpCode::ADV => self.a = self.a / 2_i64.pow(op.apply_combo_operand(&self) as u32), // This getting to big could crash the program. Integer division truncates.
            OpCode::BXL => self.b = self.b ^ op.operand,
            OpCode::BST => self.b = op.apply_combo_operand(&self).rem_euclid(8),
            OpCode::JNZ => {
                if self.a != 0 {
                    *instruction_pointer = op.operand;
                    *instruction_pointer -= 1;
                }
            }
            OpCode::BXC => self.b = self.b ^ self.c,
            OpCode::OUT => {
                output_string
                    .push_str(&format!("{},", op.apply_combo_operand(&self).rem_euclid(8)));
            }
            OpCode::BDV => self.b = self.a / 2_i64.pow(op.apply_combo_operand(&self) as u32),
            OpCode::CDV => self.c = self.a / 2_i64.pow(op.apply_combo_operand(&self) as u32),
        }
        *instruction_pointer += 1;
    }
}

#[derive(Debug)]
struct ProgramIteration {
    operand: i64,
    opcode: OpCode,
}
impl ProgramIteration {
    fn apply_combo_operand(&self, registers: &Registers) -> i64 {
        match self.operand {
            0..=3 => self.operand,
            4 => registers.a,
            5 => registers.b,
            6 => registers.c,
            _ => panic!("Not a valid program!"),
        }
    }
}

#[derive(Debug)]
enum OpCode {
    ADV = 0,
    BXL = 1,
    BST = 2,
    JNZ = 3,
    BXC = 4,
    OUT = 5,
    BDV = 6,
    CDV = 7,
}
impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::ADV,
            1 => Self::BXL,
            2 => Self::BST,
            3 => Self::JNZ,
            4 => Self::BXC,
            5 => Self::OUT,
            6 => Self::BDV,
            7 => Self::CDV,
            _ => panic!("Unknown value: {}", value),
        }
    }
}
