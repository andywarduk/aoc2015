use memmap2::Mmap;
use std::collections::{HashMap, HashSet, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use std::{fs::File, io::{BufRead, BufReader}};

type Replacements = HashMap<String, Vec<String>>;
type Replacement = HashMap<String, String>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input19.txt")?;

    let (fwd_replacements, bwd_replacements, molecule) = parse_lines(&lines);

    part1(&molecule, &fwd_replacements);

    part2(&molecule, &fwd_replacements, &bwd_replacements);

    Ok(())
}

fn part1(molecule: &str, fwd_replacements: &Replacements) {
    let mut hashes = HashSet::new();
    let mut rep_count: u32 = 0;

    let elements = split_molecule(molecule);

    for i in 0..elements.len() {
        if let Some(replacements) = fwd_replacements.get(&elements[i]) {
            for r in replacements {
                let new_molecule: String = elements.iter().enumerate().fold("".to_string(), |acc, (idx, e)| {
                    if idx == i {
                        acc + r
                    } else {
                        acc + e
                    }
                });

                let mut hasher = DefaultHasher::new();
                new_molecule.hash(&mut hasher);
                let hash = hasher.finish();

                hashes.insert(hash);

                rep_count += 1;
            }
        }
    }

    println!("Unique molecules from {} replacements (part 1): {}", rep_count, hashes.len());
}

fn split_molecule(molecule: &str) -> Vec<String> {
    let mut elements = Vec::new();
    let chars: Vec<char> = molecule.chars().collect();

    let mut pos = 0;
    while pos < chars.len() {
        assert!(chars[pos].is_uppercase());

        pos += if pos + 1 < chars.len() && chars[pos + 1].is_lowercase() {
            elements.push(chars[pos..pos + 2].iter().collect());
            2
        } else {
            elements.push(chars[pos].to_string());
            1
        }
    }

    elements
}

fn part2(molecule: &str, fwd_replacements: &Replacements, bwd_replacements: &Replacement) {
    let mut min_rep = usize::MAX;

    let mut terminals = Replacement::new();
    let mut non_terminals = Replacement::new();

    // Split bwd_replacements in to terminals and non-terminals
    for (to, from) in bwd_replacements {
        let split = split_molecule(to);
        let mut terminal = false;

        for s in split {
            if fwd_replacements.get(&s).is_none() {
                terminal = true;
                break
            }
        }

        if terminal {
            terminals.insert(to.to_string(), from.clone());
        } else {
            non_terminals.insert(to.to_string(), from.clone());
        }
    }

    part2_iter(molecule.to_string(), &terminals, &non_terminals, 0, &mut min_rep);

    println!("Minimum replacements (part 2): {}", min_rep);
}

fn part2_iter(molecule: String, terminals: &Replacement, non_terminals: &Replacement, mut no_rep: usize, min_rep: &mut usize) {
    if no_rep >= *min_rep {
        return
    }

    if molecule == "e" {
        println!("e {} at {}", molecule, no_rep);
        *min_rep = no_rep;
        return
    }

    // Replace terminals
    let mut new_molecule1 = molecule.clone();
    for (to, from) in terminals {
        let trep = new_molecule1.match_indices(to).count();
        if trep > 0 {
            no_rep += trep;
            new_molecule1 = new_molecule1.replace(to, from);
        }
    }
    
    // Iterate non-terminals
    for (to, from) in non_terminals {
        for (idx, _) in new_molecule1.match_indices(to) {
            let mut new_molecule2 = "".to_string();
            new_molecule2.push_str(&new_molecule1[0..idx]);
            new_molecule2.push_str(from);
            new_molecule2.push_str(&new_molecule1[idx + to.len()..]);

            part2_iter(new_molecule2, terminals, non_terminals, no_rep + 1, min_rep);
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

        lines.push(line);
    }

    Ok(lines)
}

fn parse_lines(lines: &Vec<String>) -> (Replacements, Replacement, String) {
    let mut fwd_replacements: Replacements = HashMap::new();
    let mut bwd_replacements: Replacement = HashMap::new();
    let mut in_replacements = true;
    let mut molecule = "".to_string();

    for l in lines {
        if l.is_empty() {
            if in_replacements {
                in_replacements = false;
                continue
            } else {
                break
            }
        }

        if in_replacements {
            let mut split = l.split(" => ");
            let from = split.next().unwrap();
            let to = split.next().unwrap();

            if let Some(rep) = fwd_replacements.get_mut(from) {
                rep.push(to.to_string());
            } else {
                fwd_replacements.insert(from.to_string(), vec![to.to_string()]);
            }

            bwd_replacements.insert(to.to_string(), from.to_string());
        } else {
            molecule = l.clone();
            break
        }
    }

    (fwd_replacements, bwd_replacements, molecule)
}
