use memmap::Mmap;
use std::{collections::HashMap, fs::File};
use std::str;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let directions = load_input("input03.txt")?;

    part1(directions);

    part2(directions);

    Ok(())
}

fn part1(directions: &str) {
    let mut visits = HashMap::new();

    let mut x = 0;
    let mut y = 0;

    let mut visit = |xadd, yadd| {
        x += xadd;
        y += yadd;

        let key = format!("{}x{}", x, y);

        if let Some(v) = visits.get_mut(&key) {
            *v += 1;
        } else {
            visits.insert(key, 1);
        }
    };

    visit(0,0);

    for d in directions.chars() {
        match d {
            '<' => visit(-1, 0),
            '>' => visit(1, 0),
            '^' => visit(0, -1),
            'v' => visit(0, 1),
            _ => {}
        }
    }

    println!("{} houses visited (part 1)", visits.len());
}

fn part2(directions: &str) {
    let mut visits = HashMap::new();

    let mut x: [i32; 2] = [0; 2];
    let mut y: [i32; 2] = [0; 2];
    let mut turn = 1;

    let mut visit = |xadd, yadd| {
        x[turn] += xadd;
        y[turn] += yadd;

        let x = x[turn];
        let y = y[turn];

        turn = (turn + 1) % 2;

        let key = format!("{}x{}", x, y);

        if let Some(v) = visits.get_mut(&key) {
            *v += 1;
        } else {
            visits.insert(key, 1);
        }
    };

    visit(0,0);

    for d in directions.chars() {
        match d {
            '<' => visit(-1, 0),
            '>' => visit(1, 0),
            '^' => visit(0, -1),
            'v' => visit(0, 1),
            _ => {}
        }
    }

    println!("{} houses visited (part 2)", visits.len());
}

fn load_input(file: &str) -> Result<&str, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Convert mmap to str
    let content = unsafe { std::mem::transmute::<Mmap, &str>(mmap) };

    Ok(content)
}
