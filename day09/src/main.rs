use memmap::Mmap;
use regex::Regex;
use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input09.txt")?;

    let (nodes, edges) = parse_edges(&lines);

    process(&nodes, &edges);

    Ok(())
}

fn process(nodes: &HashSet<String>, edges: &Vec<Edge>) {
    let start_list: Vec<String> = nodes.iter().cloned().collect();

    let mut lowest_dist = usize::MAX;
    let mut highest_dist = 0;

    for start in start_list.iter() {
        walk_tree(start, nodes, edges, &Vec::new(), 0, &mut lowest_dist, &mut highest_dist);
    }

    println!("Lowest distance (part 1): {}", lowest_dist);
    println!("Highest distance (part 2): {}", highest_dist);
}

fn walk_tree(next: &String, parent_nodes: &HashSet<String>, edges: &Vec<Edge>,
        parent_path: &[String], dist: usize,
        lowest_dist: &mut usize, highest_dist: &mut usize) {
    // Mark as visited
    let mut nodes = parent_nodes.clone();
    nodes.remove(next);

    // Copy path and add next
    let mut path = parent_path.to_vec();
    path.push(next.clone());

    if nodes.is_empty() {
        // Finished
        println!("Route: {}  Distance: {}", path.join(" -> "), dist);

        if dist < *lowest_dist {
            *lowest_dist = dist;
        }

        if dist > *highest_dist {
            *highest_dist = dist;
        }
    }

    // Find outbound routes
    let choices = edges.iter().filter_map(|e| {
        if e.node1 == *next {
            if nodes.get(&e.node2).is_some() {
                return Some((e.node2.clone(), e.dist))
            }
        } else if e.node2 == *next && nodes.get(&e.node1).is_some() {
            return Some((e.node1.clone(), e.dist))
        }

        None
    });

    for (dest, dest_dist) in choices {
        walk_tree(&dest, &nodes, edges,
            &path, dist + dest_dist,
            lowest_dist, highest_dist);
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

struct Edge {
    node1: String,
    node2: String,
    dist: usize
}

fn parse_edges(lines: &Vec<String>) -> (HashSet<String>, Vec<Edge>) {
    let mut edges = Vec::new();
    let mut nodes = HashSet::new();

    let re = Regex::new(r"^(.*) to (.*) = (.*)").unwrap();

    for l in lines {
        let caps = re.captures(l).unwrap();

        edges.push(Edge {
            node1: caps[1].to_string(),
            node2: caps[2].to_string(),
            dist: caps[3].parse::<usize>().unwrap()
        });

        if nodes.get(&caps[1]).is_none() {
            nodes.insert(caps[1].to_string());
        }

        if nodes.get(&caps[2]).is_none() {
            nodes.insert(caps[2].to_string());
        }
    }

    (nodes, edges)
}