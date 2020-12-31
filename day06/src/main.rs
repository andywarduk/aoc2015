use memmap::Mmap;
use regex::Regex;
use std::{cmp::max, fs::File, io::{BufRead, BufReader}};
use std::str;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input06.txt")?;

    let instructions = parse_instructions(lines);

    part1(&instructions);

    part2(&instructions);

    Ok(())
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

        if line != "" {
            lines.push(line);
        }
    }

    Ok(lines)
}

fn part1(instructions: &Vec<Instruction>) {
    let mut board = Vec::with_capacity(1000);

    // Construct light array
    for _ in 0..1000 {
        let mut row = Vec::with_capacity(1000);

        for _ in 0..1000 {
            row.push(' ');
        }

        board.push(row);
    }

    for i in instructions {
        for y in i.y1..=i.y2 {
            for x in i.x1..=i.x2 {
                board[y][x] = match i.action {
                    Action::TurnOff => ' ',
                    Action::TurnOn => '*',
                    Action::Toggle => {
                        match board[y][x] {
                            ' ' => '*',
                            '*' => ' ',
                            _ => { panic!("Invalid board state") }
                        }
                    }
                }
            }
        }
    }

    let lit = board.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|&&c| c == '*').count()
    });

    println!("{} bulbs lit (part 1)", lit);
}

fn part2(instructions: &Vec<Instruction>) {
    let mut board = Vec::with_capacity(1000);

    // Construct light array
    for _ in 0..1000 {
        let mut row = Vec::with_capacity(1000);

        for _ in 0..1000 {
            row.push(0);
        }

        board.push(row);
    }

    for i in instructions {
        for y in i.y1..=i.y2 {
            for x in i.x1..=i.x2 {
                board[y][x] = max(0, board[y][x] + match i.action {
                    Action::TurnOff => -1,
                    Action::TurnOn => 1,
                    Action::Toggle => 2
                });
            }
        }
    }

    let brightness = board.iter().fold(0, |acc, row| {
        let sum: i32 = row.iter().sum();
        acc + sum
    });

    println!("Total brightness is {} (part 2)", brightness);
}

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize
}

fn parse_instructions(lines: Vec<String>) -> Vec<Instruction> {
    let re = Regex::new(r"^(.*) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    lines.iter().map(|l| {
        let caps = re.captures(l).unwrap();

        let action = match &caps[1] {
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            "toggle" => Action::Toggle,
            _ => { panic!("Unrecognised action {}", &caps[1]) }
        };

        Instruction {
            action,
            x1: caps[2].parse::<usize>().unwrap(),
            y1: caps[3].parse::<usize>().unwrap(),
            x2: caps[4].parse::<usize>().unwrap(),
            y2: caps[5].parse::<usize>().unwrap(),
        }
    }).collect()
}
