use memmap2::Mmap;
use regex::Regex;
use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input13.txt")?;

    let (nodes, edges) = parse_edges(&lines);

    println!("-- part 1 --");
    let optimal_happiness1 = part1(&nodes, &edges);

    println!("-- part 2 --");
    let optimal_happiness2 = part2(&nodes, &edges);
    
    println!("------------");

    println!("Optimal happiness (part 1): {}", optimal_happiness1);
    println!("Optimal happiness (part 2): {}", optimal_happiness2);

    Ok(())
}

fn part1(nodes: &[String], edges: &HashMap<String, Edge>) -> i32 {
    let mut order = nodes.to_vec();
    let remain = order.split_off(1);
    let mut optimal_happiness = 0;

    process_combinations(order, remain, edges, &mut optimal_happiness);

    optimal_happiness
}

fn part2(nodes: &Vec<String>, edges: &HashMap<String, Edge>) -> i32 {
    let me = "Me".to_string();

    let mut order = nodes.clone();

    let mut new_edges = edges.clone();

    for person in nodes {
        new_edges.insert(edge_key(person, &me), 0);
        new_edges.insert(edge_key(&me, person), 0);
    }

    order.push(me);

    let remain = order.split_off(1);
    let mut optimal_happiness = 0;

    process_combinations(order, remain, &new_edges, &mut optimal_happiness);

    optimal_happiness
}

fn process_combinations(order: Vec<String>, remain: Vec<String>, edges: &HashMap<String, Edge>, optimal_happiness: &mut i32) {
    if remain.is_empty() {
        let happiness = calc_happiness(&order, edges);

        println!("{}, happiness {}", order.join(" - "), happiness);

        if happiness > *optimal_happiness {
            *optimal_happiness = happiness;
        }
    } else {
        for person in &remain {
            let mut new_order = order.clone();
            new_order.push(person.clone());

            let new_remain = remain.iter().filter_map(|r| {
                if *r == *person {
                    None
                } else {
                    Some(r.clone())
                }
            }).collect();

            process_combinations(new_order, new_remain, edges, optimal_happiness);
        }
    }
}

fn calc_happiness(order: &Vec<String>, edges: &HashMap<String, Edge>) -> i32 {
    let mut happiness: i32 = 0;

    for i in 1..order.len() {
        happiness += happiness_sum(&order[i - 1], &order[i], edges);
    }

    happiness += happiness_sum(&order[0], &order[order.len() - 1], edges);

    happiness
}

fn happiness_sum(person1: &str, person2: &str, edges: &HashMap<String, Edge>) -> i32 {
    happiness_pair(person1, person2, edges) + happiness_pair(person2, person1, edges)
}

fn happiness_pair(person1: &str, person2: &str, edges: &HashMap<String, Edge>) -> i32 {
    let happiness = edges.get(&edge_key(person1, person2)).unwrap();

    *happiness
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

type Edge = i32;

fn parse_edges(lines: &Vec<String>) -> (Vec<String>, HashMap<String, Edge>) {
    let mut edges = HashMap::new();
    let mut nodeset = HashSet::new();

    let re = Regex::new(r"^(.*) would (.*) (.*) happiness units by sitting next to (.*).").unwrap();

    for l in lines {
        let caps = re.captures(l).unwrap();

        let mut happiness: i32 = caps[3].parse().unwrap();

        match &caps[2] {
            "gain" => {}
            "lose" => happiness = -happiness,
            _ => panic!("Unrecognised term {}", &caps[2])
        }

        edges.insert(edge_key(&caps[1], &caps[4]), happiness);

        if nodeset.get(&caps[1]).is_none() {
            nodeset.insert(caps[1].to_string());
        }
    }

    let mut nodes: Vec<String> = nodeset.iter().cloned().collect();
    nodes.sort();

    (nodes, edges)
}

fn edge_key(person1: &str, person2: &str) -> String {
    format!("{}-{}", person1, person2)
}
