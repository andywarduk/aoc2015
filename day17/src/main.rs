use memmap::Mmap;
use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input17.txt")?;

    let mut capacities: Vec<u16> = lines.iter().map(|l| l.parse().unwrap()).collect::<Vec<u16>>();
    capacities.sort();
 
    process(&capacities);

    Ok(())
}

fn process(capacities: &Vec<u16>) {
    let mut answers = Vec::new();

    walk_capacities(capacities, 150, 0, 0, 0, &mut answers);

    for a in &answers {
        println!("{}", capacity_list(capacities, *a));
    }

    println!("{} combinations (part 1)", answers.len());

    let min_bits = answers.iter().fold(u32::MAX, |min, a| {
        let bit_count = a.count_ones();

        if bit_count < min {
            bit_count
        } else {
            min
        }
    });

    let min_combinations = answers.iter().filter(|a| {
        a.count_ones() == min_bits
    }).count();

    println!("Combinations using {} containers (part 2): {}", min_bits, min_combinations);
}

fn walk_capacities(capacities: &Vec<u16>, target: u16, filled: u16, used_elems: usize, used_bits: u64, answers: &mut Vec<u64>) {
    for i in used_elems..capacities.len() {
        let next_filled = filled + capacities[i];
        let next_used_bits = used_bits | 1 << i;

        if next_filled >= target {
            if next_filled == target {
                answers.push(next_used_bits);
            } else {
                break
            }
        }

        walk_capacities(capacities, target, next_filled, i + 1, next_used_bits, answers);
    }
}

#[test]
fn test_walk_capacities() {
    let mut capacities: Vec<u16> = vec![20, 15, 10, 5, 5];
    capacities.sort();

    let mut answers = Vec::new();
    
    walk_capacities(&capacities, 25, 0, 0, 0, &mut answers);

    assert!(answers.len() == 4);

    let lists: Vec<String> = answers.iter().map(|a| capacity_list(&capacities, *a)).collect();

    assert!(lists[0] == "5+5+15");
    assert!(lists[1] == "5+20");
    assert!(lists[2] == "5+20");
    assert!(lists[3] == "10+15");
}

fn capacity_list(capacities: &[u16], used_bits: u64) -> String {
    let mut capacity_list: String = String::from("");

    let mut bits = used_bits;
    let mut bit = 1;
    let mut idx = 0;

    while bits != 0 {
        if bits & bit != 0 {
            bits &= !bit;

            if capacity_list.is_empty() {
                capacity_list = format!("{}", capacities[idx]);
            } else {
                capacity_list = format!("{}+{}", capacity_list, capacities[idx]);
            }
        }

        bit <<= 1;
        idx += 1;
    }

    capacity_list
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
