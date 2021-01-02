use memmap::Mmap;
use regex::Regex;
use std::{fs::File, io::{BufRead, BufReader}, ops::{AddAssign, Mul}};

const TEASPOONS: usize = 100;
const CALORIE_TARGET: usize = 500;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input15.txt")?;

    let ingredients = parse_ingredients(&lines);

    process(&ingredients);

    Ok(())
}

fn process(ingredients: &Vec<Qtys>) {
    let mut max_total = 0;
    let mut max_calorie_total = 0;

    for quantities in partitioner(ingredients.len(), TEASPOONS) {
        let mut tot_qtys: Qtys = Default::default();

        for (qty, ingredient) in quantities.iter().zip(ingredients) {
            tot_qtys += ingredient * *qty as i32;
        }

        let total = tot_qtys.quality_product();

        if total > max_total {
            max_total = total
        }

        if tot_qtys.calories == CALORIE_TARGET as i32 {
            if total > max_calorie_total {
                max_calorie_total = total
            }
        }
    }

    println!("Max total (part 1): {}", max_total);
    println!("Max calorie target total (part 2): {}", max_calorie_total);
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

#[derive(Default)]
struct Qtys {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}

impl Qtys {
    fn quality_product(&self) -> i32 {
        if self.capacity < 0 || self.durability < 0 || self.flavor < 0 || self.texture < 0 {
            0
        } else {
            self.capacity * self.durability * self.flavor * self.texture
        }
    }
}

impl AddAssign for Qtys {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            capacity: self.capacity + other.capacity,
            durability: self.durability + other.durability,
            flavor: self.flavor + other.flavor,
            texture: self.texture + other.texture,
            calories: self.calories + other.calories,
        };
    }
}

impl Mul<i32> for &Qtys {
    type Output = Qtys;

    fn mul(self, rhs: i32) -> Self::Output {
        Qtys {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

fn parse_ingredients(lines: &Vec<String>) -> Vec<Qtys> {
    let mut ingredients = Vec::new();

    let re = Regex::new(r"^(.*): capacity (.*), durability (.*), flavor (.*), texture (.*), calories (.*)").unwrap();

    for l in lines {
        let caps = re.captures(l).unwrap();

        ingredients.push(Qtys {
            capacity: caps[2].parse().unwrap(),
            durability: caps[3].parse().unwrap(),
            flavor: caps[4].parse().unwrap(),
            texture: caps[5].parse().unwrap(),
            calories: caps[6].parse().unwrap()
        });
    }

    ingredients
}

struct Partitioner {
    vector: Vec<usize>,
    partitions: usize,
    size: usize
}

impl Iterator for Partitioner {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        let mut item = self.partitions - 2;

        loop {
            self.vector[item] += 1;

            let sum: usize = self.vector.iter().take(self.partitions - 1).sum();

            if sum < self.size {
                self.vector[self.partitions - 1] = self.size - sum;
                break
            }

            if item == 0 {
                return None
            }

            self.vector[item] = 1;
            item -= 1;
        }
    
        Some(self.vector.clone())
    }
}

fn partitioner(partitions: usize, size: usize) -> Partitioner {
    let mut vector = Vec::with_capacity(partitions);

    for _ in 0..partitions - 2 {
        vector.push(1)
    }
    vector.push(0);
    vector.push(0);

    Partitioner {
        vector,
        partitions,
        size,
    }
}
