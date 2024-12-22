use anyhow::anyhow;
use anyhow::Result;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Instruction {
    Adv(Combo),
    Bxl(usize),
    Bst(Combo),
    Jnz(usize),
    Bxc,
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

#[derive(Debug)]
enum Combo {
    Literal(usize),
    Register(Register),
}

impl Combo {
    fn resolve_value(&self, state: &State) -> usize {
        match &self {
            Self::Literal(l) => *l,
            Self::Register(Register::A) => state.register_a,
            Self::Register(Register::B) => state.register_b,
            Self::Register(Register::C) => state.register_c,
        }
    }
}

#[derive(Debug)]
enum Register {
    A,
    B,
    C,
}

#[derive(Debug)]
struct State {
    instruction_pointer: usize,

    register_a: usize,
    register_b: usize,
    register_c: usize,

    out: Vec<usize>,
}

impl State {
    fn append_output(&mut self, number: usize) {
        self.out.push(number);
    }

    fn display_output(&self) {
        println!(
            "{}",
            self.out
                .clone()
                .into_iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}

fn parse_combo(number: usize) -> Result<Combo> {
    match number {
        0 | 1 | 2 | 3 => Ok(Combo::Literal(number)),
        4 => Ok(Combo::Register(Register::A)),
        5 => Ok(Combo::Register(Register::B)),
        6 => Ok(Combo::Register(Register::C)),
        _ => Err(anyhow!("invalid operand: 7")),
    }
}

fn parse_instruction(opcode: usize, operand: usize) -> Result<Instruction> {
    match opcode {
        0 => Ok(Instruction::Adv(parse_combo(operand)?)),
        1 => Ok(Instruction::Bxl(operand)),
        2 => Ok(Instruction::Bst(parse_combo(operand)?)),
        3 => Ok(Instruction::Jnz(operand)),
        4 => Ok(Instruction::Bxc),
        5 => Ok(Instruction::Out(parse_combo(operand)?)),
        6 => Ok(Instruction::Bdv(parse_combo(operand)?)),
        7 => Ok(Instruction::Cdv(parse_combo(operand)?)),
        _ => Err(anyhow!(format!("invalid opcode {}", opcode))),
    }
}

fn parse(program_string: &str) -> Result<Vec<Instruction>> {
    let mut result = Vec::new();

    let program_numbers: Result<Vec<usize>, _> = program_string
        .split(",")
        .map(|n| n.parse::<usize>())
        .collect();
    let program_ref = &program_numbers?;

    let mut i = 0;
    while i < program_ref.len() - 1 {
        let opcode = program_ref
            .get(i)
            .ok_or(anyhow!(format!("no opcode at index {i}")))?;
        let operand = program_ref
            .get(i + 1)
            .ok_or(anyhow!(format!("no opcode at index {i}")))?;

        result.push(parse_instruction(*opcode, *operand)?);

        i += 2;
    }

    Ok(result)
}

fn interpret(instructions: &[Instruction], state: &mut State) -> Result<()> {
    while state.instruction_pointer < instructions.len() {
        let instruction = instructions
            .get(state.instruction_pointer)
            .ok_or(anyhow!(format!(
                "no instruction at instruction pointer {}",
                state.instruction_pointer
            )))?;

        match instruction {
            Instruction::Adv(combo) => {
                state.register_a =
                    state.register_a / 2_usize.pow(combo.resolve_value(state).try_into()?);
                state.instruction_pointer += 1;
            }
            Instruction::Bxl(literal) => {
                state.register_b = state.register_b ^ literal;
                state.instruction_pointer += 1;
            }
            Instruction::Bst(combo) => {
                state.register_b = combo.resolve_value(state) % 8;
                state.instruction_pointer += 1;
            }
            Instruction::Jnz(literal) => {
                if state.register_a == 0 {
                    state.instruction_pointer += 1;
                } else {
                    state.instruction_pointer = *literal;
                }
            }
            Instruction::Bxc => {
                state.register_b = state.register_b ^ state.register_c;
                state.instruction_pointer += 1;
            }
            Instruction::Out(combo) => {
                state.append_output(combo.resolve_value(state) % 8);
                state.instruction_pointer += 1;
            }
            Instruction::Bdv(combo) => {
                state.register_b =
                    state.register_a / 2_usize.pow(combo.resolve_value(state).try_into()?);
                state.instruction_pointer += 1;
            }
            Instruction::Cdv(combo) => {
                state.register_c =
                    state.register_a / 2_usize.pow(combo.resolve_value(state).try_into()?);
                state.instruction_pointer += 1;
            }
        }
    }

    Ok(())
}

fn parse_input() -> Result<(Vec<Instruction>, State)> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let register_a = lines
        .next()
        .ok_or(anyhow!("no line for register a"))??
        .split(":")
        .nth(1)
        .ok_or(anyhow!("nothing for register a after :"))?
        .trim()
        .parse::<usize>()?;
    let register_b: usize = lines
        .next()
        .ok_or(anyhow!("no line for register b"))??
        .split(":")
        .nth(1)
        .ok_or(anyhow!("nothing for register b after :"))?
        .trim()
        .parse::<usize>()?;
    let register_c: usize = lines
        .next()
        .ok_or(anyhow!("no line for register c"))??
        .split(":")
        .nth(1)
        .ok_or(anyhow!("nothing for register c after :"))?
        .trim()
        .parse::<usize>()?;

    lines.next();

    let program_text = lines
        .next()
        .ok_or(anyhow!("no line for program text"))??
        .split(":")
        .nth(1)
        .ok_or(anyhow!("nothing for program after :"))?
        .trim()
        .to_string();

    let program = parse(&program_text)?;
    let state = State {
        instruction_pointer: 0,
        register_a,
        register_b,
        register_c,
        out: Vec::new(),
    };

    Ok((program, state))
}

fn main() -> Result<()> {
    let (program, mut state) = parse_input()?;

    for ins in &program {
        println!("{:?}", ins);
    }

    interpret(&program, &mut state)?;

    println!("--");
    println!("{:?}", state);
    state.display_output();

    Ok(())
}
