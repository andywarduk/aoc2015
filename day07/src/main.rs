use memmap::Mmap;
use regex::Regex;
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input07.txt")?;

    let part1sig = part1(&lines);

    part2(&lines, part1sig);

    Ok(())
}

fn part1(lines: &[String]) -> u16 {
    let gates = parse_gates(lines);

    let mut signals = get_signals(&gates);

    // Resolve the gates
    resolve_gates(gates, &mut signals);

    // Get signal a
    let signal_a = get_signal_value(&signals, &String::from("a")).unwrap();

    println!("Signal a is {} (part 1)", signal_a);

    signal_a
}

fn part2(lines: &[String], part1sig: u16) {
    let mut gates = parse_gates(lines);

    let mut signals = get_signals(&gates);

    // Find and remove signal b initialisation from the gates
    let elem = gates
        .iter()
        .position(|gate| matches!(gate, Gate::Signal(outs, _) if outs == "b"))
        .expect("Input b not found");

    gates.swap_remove(elem);

    // Set signal b
    set_signal_value(&mut signals, &String::from("b"), part1sig);

    // Resolve the gates
    resolve_gates(gates, &mut signals);

    println!("Signal a is {} (part 2)", get_signal_value(&signals, &String::from("a")).unwrap());
}

fn resolve_gates(mut gates: Vec<Gate>, signals: &mut SignalMap) {
    while !gates.is_empty() {
        let mut gate_no = 0;

        while gate_no < gates.len() {
            let gate = &gates[gate_no];
            let mut resolved = false;

            match gate {
                Gate::Signal(outs, ins) => {
                    if let Some(value) = get_input_value(signals, ins) {
                        set_signal_value(signals, outs, value);
                        resolved = true;
                    }
                },
                Gate::And(outs, ins1, ins2) => {
                    if let Some(value1) = get_input_value(signals, ins1) {
                        if let Some(value2) = get_input_value(signals, ins2) {
                            set_signal_value(signals, outs, value1 & value2);
                            resolved = true;
                        }
                    }
                },
                Gate::Or(outs, ins1, ins2) => {
                    if let Some(value1) = get_input_value(signals, ins1) {
                        if let Some(value2) = get_input_value(signals, ins2) {
                            set_signal_value(signals, outs, value1 | value2);
                            resolved = true;
                        }
                    }
                },
                Gate::LShift(outs, ins, bits) => {
                    if let Some(value) = get_input_value(signals, ins) {
                        set_signal_value(signals, outs, value << bits);
                        resolved = true;
                    }
                },
                Gate::RShift(outs, ins, bits) => {
                    if let Some(value) = get_input_value(signals, ins) {
                        set_signal_value(signals, outs, value >> bits);
                        resolved = true;
                    }
                },
                Gate::Not(outs, ins) => {
                    if let Some(value) = get_input_value(signals, ins) {
                        set_signal_value(signals, outs, !value);
                        resolved = true;
                    }
                }
            }

            if resolved {
                gates.swap_remove(gate_no);
            } else {
                gate_no += 1;
            }
        }
    }
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

type Out = String;

#[derive(Debug)]
enum In {
    Signal(u16),
    Input(String)
}

#[derive(Debug)]
enum Gate {
    Signal(Out, In), // out, signal
    And(Out, In, In), // Out, In1, In2
    Or(Out, In, In), // Out, In1, In2
    LShift(Out, In, u16), // Out, In, bits
    RShift(Out, In, u16), // Out, In, bits
    Not(Out, In), // Out, In
}

fn parse_gates(lines: &[String]) -> Vec<Gate> {
    let re_sig = Regex::new(r"^([a-z]+|\d+) -> ([a-z]+)$").unwrap();
    let re_andor = Regex::new(r"^([a-z]+|\d+) (AND|OR) ([a-z]+) -> ([a-z]+)$").unwrap();
    let re_sh = Regex::new(r"^([a-z]+) ([LR])SHIFT (\d+) -> ([a-z]+)$").unwrap();
    let re_not = Regex::new(r"^NOT ([a-z]+) -> ([a-z]+)$").unwrap();

    lines.iter().map(|l| {
        if let Some(caps) = re_sig.captures(l) {
            Gate::Signal(caps[2].to_string(), parse_in(&caps[1]))
        } else if let Some(caps) = re_andor.captures(l) {
            match &caps[2] {
                "AND" => Gate::And(caps[4].to_string(), parse_in(&caps[1]), parse_in(&caps[3])),
                "OR" => Gate::Or(caps[4].to_string(), parse_in(&caps[1]), parse_in(&caps[3])),
                _ => { panic!("Invalid and/or") }
            }
        } else if let Some(caps) = re_sh.captures(l) {
            match &caps[2] {
                "L" => Gate::LShift(caps[4].to_string(), parse_in(&caps[1]), caps[3].parse::<u16>().unwrap()),
                "R" => Gate::RShift(caps[4].to_string(), parse_in(&caps[1]), caps[3].parse::<u16>().unwrap()),
                _ => { panic!("Invalid shift") }
            }
        } else if let Some(caps) = re_not.captures(l) {
            Gate::Not(caps[2].to_string(), parse_in(&caps[1]))
        } else {
            panic!("Can't match {}", l)
        }
    }).collect()
}

fn parse_in(string: &str) -> In {
    if let Ok(n) = string.parse::<u16>() {
        In::Signal(n)
    } else {
        In::Input(string.to_string())
    }
}

#[derive(Debug)]
struct Signal {
    value: Option<u16>
}

type SignalMap = HashMap<String, Signal>;

fn get_signals(gates: &Vec<Gate>) -> SignalMap {
    let mut signals = SignalMap::new();

    for gate in gates {
        match gate {
            Gate::Signal(outs, ins) => {
                add_signal(&mut signals, outs);
                add_in_signal(&mut signals, ins);
            },
            Gate::And(outs, ins1, ins2) => {
                add_signal(&mut signals, outs);
                add_in_signal(&mut signals, ins1);
                add_in_signal(&mut signals, ins2);
            },
            Gate::Or(outs, ins1, ins2) => {
                add_signal(&mut signals, outs);
                add_in_signal(&mut signals, ins1);
                add_in_signal(&mut signals, ins2);
            },
            Gate::LShift(outs, ins, _) => {
                add_signal(&mut signals, outs);
                add_in_signal(&mut signals, ins);
            },
            Gate::RShift(outs, ins, _) => {
                add_signal(&mut signals, outs);
                add_in_signal(&mut signals, ins);
            },
            Gate::Not(outs, ins) => {
                add_signal(&mut signals, outs);
                add_in_signal(&mut signals, ins);
            }
        }
    }

    signals
}

fn add_signal(signals: &mut SignalMap, signal: &str) {
    signals.insert(signal.to_string(), Signal {
        value: None
    });
}

fn add_in_signal(signals: &mut SignalMap, ins: &In)  {
    if let In::Input(name) = ins {
        add_signal(signals, name);
    }
}

fn get_signal_value(signals: &SignalMap, name: &String) -> Option<u16> {
    let signal = signals.get(name).unwrap();

    signal.value
}

fn set_signal_value(signals: &mut SignalMap, name: &String, value: u16) {
    let val = signals.get_mut(name).unwrap();
    val.value = Some(value);
}

fn get_input_value(signals: &SignalMap, ins: &In) -> Option<u16> {
    match ins {
        In::Input(insig) => {
            get_signal_value(signals, insig)
        },
        In::Signal(value) => {
            Some(*value)
        }
    }
}
