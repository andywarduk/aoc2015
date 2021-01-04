use memmap::Mmap;
use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weights = load_input("input24.txt")?;

    part1(&weights);

    part2(&weights);

    Ok(())
}

fn part1(weights: &Vec<u16>) {
    println!("--- Part 1 ---");
    calc_min_qe(weights, 3);
}

fn part2(weights: &Vec<u16>) {
    println!("--- Part 2 ---");
    calc_min_qe(weights, 4);
}

fn calc_min_qe(weights: &Vec<u16>, compartments: u8) {
    let packages = weights.len();
    let total_weight = weights.iter().sum::<u16>();
    let weight_per_compartment = total_weight / compartments as u16;

    println!("Total weight: {}", total_weight);
    println!("Compartment weight: {}", weight_per_compartment);

    // Find all combinations of weights which add up to the target weight
    let mut combinations = Vec::with_capacity(8192);

    walk_weights(weights, weight_per_compartment, 0, 0, 0, &mut combinations);

    println!("Combinations: {}", combinations.len());

    // Get minimum bit count of combinations
    let min_bits = combinations.iter().map(|c| c.count_ones()).min().unwrap();

    println!("Minimum presents: {}", min_bits);

    // Filter the array by minimum bit count
    let mut min_combinations: Vec<(u32, u64)> = combinations.iter().filter_map(|&c| {
        if c.count_ones() == min_bits {
            Some((c, calc_qe(weights, c)))
        } else {
            None
        }
    }).collect();

    // Sort the array by QE
    min_combinations.sort_by(|&(_, qe1), &(_, qe2)| {
        qe1.cmp(&qe2)
    });

    for (bits, qe) in min_combinations {
        let mut other_comp = Vec::new();

        if check_combination(&combinations, bits, compartments, packages, &mut other_comp) {
            println!("Minimum QE: {}", qe);

            print!("Compartments: {}", weight_list(weights, bits));
            for other in other_comp {
                print!(", {}", weight_list(weights, other));
            }
            println!("");

            break
        }
    }
}

fn check_combination(combinations: &Vec<u32>, bits: u32, compartments: u8, packages: usize, other_comp: &mut Vec<u32>) -> bool {
    let full_mask: u32 = (1 << packages) - 1;

    check_combination_iter(combinations, bits, full_mask, other_comp,0, compartments as usize - 2)
}

fn check_combination_iter(combinations: &Vec<u32>, bits: u32, full_mask: u32, other_comp: &mut Vec<u32>,
        start: usize, depth: usize) -> bool {

    for i in start..combinations.len() - depth {
        let filled = combinations[i];

        // Check for overlaps
        if bits & filled != 0 {
            continue
        };

        // This one fits
        let cur_mask = bits | filled;

        if depth == 0 {
            if cur_mask == full_mask {
                other_comp.push(filled);
                return true
            }
        } else {
            if check_combination_iter(combinations, cur_mask, full_mask, other_comp, i + 1, depth - 1) {
                other_comp.push(filled);
                return true
            }
        }
    }

    false
}

fn walk_weights(weights: &Vec<u16>, target: u16, filled: u16, used_elems: usize, used_bits: u32, answers: &mut Vec<u32>) {
    for i in used_elems..weights.len() {
        let next_filled = filled + weights[i];
        let next_used_bits = used_bits | 1 << i;

        if next_filled >= target {
            if next_filled == target {
                answers.push(next_used_bits);
            } else {
                break
            }
        }

        walk_weights(weights, target, next_filled, i + 1, next_used_bits, answers);
    }
}

fn weight_list(weights: &Vec<u16>, used_bits: u32) -> String {
    let mut weight_list: String = String::from("");

    let mut bits = used_bits;
    let mut bit = 1;
    let mut idx = 0;

    while bits != 0 {
        if bits & bit != 0 {
            bits &= !bit;

            if weight_list == "" {
                weight_list = format!("{}", weights[idx]);
            } else {
                weight_list = format!("{}+{}", weight_list, weights[idx]);
            }
        }

        bit <<= 1;
        idx += 1;
    }

    weight_list
}

fn calc_qe(weights: &Vec<u16>, used_bits: u32) -> u64 {
    let mut result = 1;

    let mut bits = used_bits;
    let mut bit = 1;
    let mut idx = 0;

    while bits != 0 {
        if bits & bit != 0 {
            bits &= !bit;
            result *= weights[idx] as u64;
        }

        bit <<= 1;
        idx += 1;
    }

    result
}

fn load_input(file: &str) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Create buf reader for mmapped file
    let buf_reader = BufReader::new(mmap.as_ref());

    let mut weights = Vec::new();

    // Iterate lines
    for line_res in buf_reader.lines() {
        let line = line_res?;

        if line != "" {
            weights.push(line.parse::<u16>().unwrap());
        }
    }

    Ok(weights)
}
