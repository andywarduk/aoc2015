use memmap::Mmap;
use std::{fs::File, io::{BufRead, BufReader}};

const TEST_RESULTS: Sue = Sue {
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1)
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input16.txt")?;

    let sues = parse_sues(&lines);

    part1(&sues);

    part2(&sues);

    Ok(())
}

fn part1(sues: &[Sue]) {
    let valid_sues: Vec<usize> = sues.iter().enumerate().filter_map(|(idx, sue)| {
        if match_sue1(sue) {
            Some(idx)
        } else {
            None
        }
    }).collect();

    assert!(valid_sues.len() == 1);
    println!("Valid Sue (part 1): {}", valid_sues[0] + 1);
}

fn part2(sues: &[Sue]) {
    let valid_sues: Vec<usize> = sues.iter().enumerate().filter_map(|(idx, sue)| {
        if match_sue2(sue) {
            Some(idx)
        } else {
            None
        }
    }).collect();

    assert!(valid_sues.len() == 1);
    println!("Valid Sue (part 2): {}", valid_sues[0] + 1);
}

fn match_sue1(sue: &Sue) -> bool {
    if let Some(children) = sue.children {
        if children != TEST_RESULTS.children.unwrap() { return false }
    }
    if let Some(cats) = sue.cats {
        if cats != TEST_RESULTS.cats.unwrap() { return false }
    }
    if let Some(samoyeds) = sue.samoyeds {
        if samoyeds != TEST_RESULTS.samoyeds.unwrap() { return false }
    }
    if let Some(pomeranians) = sue.pomeranians {
        if pomeranians != TEST_RESULTS.pomeranians.unwrap() { return false }
    }
    if let Some(akitas) = sue.akitas {
        if akitas != TEST_RESULTS.akitas.unwrap() { return false }
    }
    if let Some(vizslas) = sue.vizslas {
        if vizslas != TEST_RESULTS.vizslas.unwrap() { return false }
    }
    if let Some(goldfish) = sue.goldfish {
        if goldfish != TEST_RESULTS.goldfish.unwrap() { return false }
    }
    if let Some(trees) = sue.trees {
        if trees != TEST_RESULTS.trees.unwrap() { return false }
    }
    if let Some(cars) = sue.cars {
        if cars != TEST_RESULTS.cars.unwrap() { return false }
    }
    if let Some(perfumes) = sue.perfumes {
        if perfumes != TEST_RESULTS.perfumes.unwrap() { return false }
    }

    true
}

fn match_sue2(sue: &Sue) -> bool {
    if let Some(children) = sue.children {
        if children != TEST_RESULTS.children.unwrap() { return false }
    }
    if let Some(cats) = sue.cats {
        if cats <= TEST_RESULTS.cats.unwrap() { return false }
    }
    if let Some(samoyeds) = sue.samoyeds {
        if samoyeds != TEST_RESULTS.samoyeds.unwrap() { return false }
    }
    if let Some(pomeranians) = sue.pomeranians {
        if pomeranians >= TEST_RESULTS.pomeranians.unwrap() { return false }
    }
    if let Some(akitas) = sue.akitas {
        if akitas != TEST_RESULTS.akitas.unwrap() { return false }
    }
    if let Some(vizslas) = sue.vizslas {
        if vizslas != TEST_RESULTS.vizslas.unwrap() { return false }
    }
    if let Some(goldfish) = sue.goldfish {
        if goldfish >= TEST_RESULTS.goldfish.unwrap() { return false }
    }
    if let Some(trees) = sue.trees {
        if trees <= TEST_RESULTS.trees.unwrap() { return false }
    }
    if let Some(cars) = sue.cars {
        if cars != TEST_RESULTS.cars.unwrap() { return false }
    }
    if let Some(perfumes) = sue.perfumes {
        if perfumes != TEST_RESULTS.perfumes.unwrap() { return false }
    }

    true
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

#[derive(Default)]
struct Sue {
    children: Option<u16>,
    cats: Option<u16>,
    samoyeds: Option<u16>,
    pomeranians: Option<u16>,
    akitas: Option<u16>,
    vizslas: Option<u16>,
    goldfish: Option<u16>,
    trees: Option<u16>,
    cars: Option<u16>,
    perfumes: Option<u16>,
}

fn parse_sues(lines: &Vec<String>) -> Vec<Sue> {
    let mut sues = Vec::new();

    for l in lines {
        let termstr: String = l.split(": ").skip(1).collect::<Vec<&str>>().join(" ");
        let terms: Vec<_> = termstr.split(", ").collect();

        let mut sue: Sue = Default::default();

        for t in terms {
            let kv: Vec<_> = t.split(' ').collect();

            match kv[0] {
                "children" => sue.children = Some(kv[1].parse::<u16>().unwrap()),
                "cats" => sue.cats = Some(kv[1].parse::<u16>().unwrap()),
                "samoyeds" => sue.samoyeds = Some(kv[1].parse::<u16>().unwrap()),
                "pomeranians" => sue.pomeranians = Some(kv[1].parse::<u16>().unwrap()),
                "akitas" => sue.akitas = Some(kv[1].parse::<u16>().unwrap()),
                "vizslas" => sue.vizslas = Some(kv[1].parse::<u16>().unwrap()),
                "goldfish" => sue.goldfish = Some(kv[1].parse::<u16>().unwrap()),
                "trees" => sue.trees = Some(kv[1].parse::<u16>().unwrap()),
                "cars" => sue.cars = Some(kv[1].parse::<u16>().unwrap()),
                "perfumes" => sue.perfumes = Some(kv[1].parse::<u16>().unwrap()),
                _ => panic!("term {} not recognised", kv[0])
            }
        }

        sues.push(sue);
    }

    sues
}
