use memmap2::Mmap;
use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input23.txt")?;

    let instructions = parse_instructions(&lines);

    part1(&instructions);
    part2(&instructions);

    Ok(())
}

fn part1(instructions: &Vec<Instruction>) {
    let state = run_program(instructions, 0);

    println!("b register (part 1): {}", state.b);
}

fn part2(instructions: &Vec<Instruction>) {
    let state = run_program(instructions, 1);

    println!("b register (part 2): {}", state.b);
}

#[derive(Debug)]
enum Instruction {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(i8),
    Jie(char, i8),
    Jio(char, i8)
}

#[derive(Default, Debug)]
struct State {
    a: u32,
    b: u32,
    pc: i16
}

impl State {
    fn getreg(&self, reg: char) -> u32 {
        match reg {
            'a' => self.a,
            'b' => self.b,
            _ => panic!("Unrecognised register {}", reg)
        }
    }

    fn setreg(&mut self, reg: char, value: u32) {
        match reg {
            'a' => self.a = value,
            'b' => self.b = value,
            _ => panic!("Unrecognised register {}", reg)
        }
    }
}

fn run_program(instructions: &Vec<Instruction>, a_reg: u32) -> State {
    let mut state: State = State {
        a: a_reg,
        ..Default::default()
    };

    while state.pc >= 0 && state.pc < instructions.len() as i16 {
        let instruction = &instructions[state.pc as usize];

        // println!("{}: {:?}", state.pc, instruction);

        match instruction {
            Instruction::Hlf(reg) => state.setreg(*reg, state.getreg(*reg) / 2),
            Instruction::Tpl(reg) => state.setreg(*reg, state.getreg(*reg) * 3),
            Instruction::Inc(reg) => state.setreg(*reg, state.getreg(*reg) + 1),
            Instruction::Jmp(offset) => state.pc += *offset as i16 - 1,
            Instruction::Jie(reg, offset) => if state.getreg(*reg) % 2 == 0 { state.pc += *offset as i16 - 1 },
            Instruction::Jio(reg, offset) => if state.getreg(*reg) == 1 { state.pc += *offset as i16 - 1 },
        }

        // println!("{:?}", state);

        state.pc += 1;
    }

    state
}

fn parse_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for l in lines {
        let mut terms = l.split_whitespace();

        match terms.next().unwrap() {
            "hlf" => instructions.push(Instruction::Hlf(terms.next().unwrap().chars().next().unwrap())),
            "tpl" => instructions.push(Instruction::Tpl(terms.next().unwrap().chars().next().unwrap())),
            "inc" => instructions.push(Instruction::Inc(terms.next().unwrap().chars().next().unwrap())),
            "jmp" => instructions.push(Instruction::Jmp(terms.next().unwrap().parse::<i8>().unwrap())),
            "jie" => instructions.push(Instruction::Jie(terms.next().unwrap().chars().next().unwrap(),
                terms.next().unwrap().parse::<i8>().unwrap())),
            "jio" => instructions.push(Instruction::Jio(terms.next().unwrap().chars().next().unwrap(),
                terms.next().unwrap().parse::<i8>().unwrap())),
            _ => panic!("Unrecognised instruction {}", l)
        }
    }

    instructions
}

fn load_input(file: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Create buf reader for mmapped file
    let buf_reader = BufReader::new(mmap.as_ref());

    let mut lines = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if !line.is_empty() {
            lines.push(line);
        }
    }

    Ok(lines)
}
