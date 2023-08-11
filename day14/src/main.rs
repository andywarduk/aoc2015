use memmap::Mmap;
use regex::Regex;
use std::{cmp::{min, Ordering}, fs::File, io::{BufRead, BufReader}};

const RACE_DUR: u32 = 2503;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input14.txt")?;

    let reindeer = parse_reindeer(&lines);

    part1(&reindeer);

    part2(&reindeer);

    Ok(())
}

fn part1(reindeer: &[Reindeer]) {
    let dists: Vec<u32> = reindeer.iter().map(|r| {
        let tot_times = RACE_DUR / r.total_dur;
        let remainder = RACE_DUR % r.total_dur;

        let mut dist = tot_times * r.speed * r.travel_dur;
        dist += r.speed * min(r.travel_dur, remainder);

        dist
    }).collect();

    println!("Distances: {:?}", dists);

    println!("Max distance (part 1): {}", dists.iter().max().unwrap());
}

struct ReindeerState<'a> {
    moving: bool,
    time_left: u32,
    dist: u32,
    points: u32,
    reindeer: &'a Reindeer,
}

fn part2(reindeer: &[Reindeer]) {
    let mut state: Vec<ReindeerState> = reindeer.iter().map(|r| {
        ReindeerState {
            moving: true,
            time_left: r.travel_dur,
            dist: 0,
            points: 0,
            reindeer: r,
        }
    }).collect();

    for _ in 0..RACE_DUR {
        // Calculate next seconds
        for rs in &mut state {
            if rs.moving {
                rs.dist += rs.reindeer.speed;
            }

            rs.time_left -= 1;
            if rs.time_left == 0 {
                if rs.moving {
                    rs.moving = false;
                    rs.time_left = rs.reindeer.rest_dur;
                } else {
                    rs.moving = true;
                    rs.time_left = rs.reindeer.travel_dur;
                }
            }
        }

        // Find out who is winning
        let (leaders, _) = state.iter().enumerate()
            .fold((Vec::new(), 0), |(mut leaders, lead_dist), (idx, rs)| {
                match rs.dist.cmp(&lead_dist) {
                    Ordering::Greater => (vec![idx], rs.dist),
                    Ordering::Equal => {
                        leaders.push(idx);
                        (leaders, lead_dist)    
                    }
                    Ordering::Less => (leaders, lead_dist),
                }
            }
        );

        for leader in leaders {
            state[leader].points += 1;
        }
    }

    println!("Points: {:?}", state.iter().map(|rs| rs.points).collect::<Vec<u32>>());

    println!("Max points (part 2): {}", state.iter().map(|rs| rs.points).max().unwrap());
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

struct Reindeer {
    speed: u32,
    travel_dur: u32,
    rest_dur: u32,
    total_dur: u32
}

fn parse_reindeer(lines: &Vec<String>) -> Vec<Reindeer> {
    let mut reindeer = Vec::new();

    let re = Regex::new(r"^.* can fly (.*) km/s for (.*) seconds, but then must rest for (.*) seconds.").unwrap();

    for l in lines {
        let caps = re.captures(l).unwrap();

        let travel_dur = caps[2].parse().unwrap();
        let rest_dur = caps[3].parse().unwrap();

        reindeer.push(Reindeer {
            speed: caps[1].parse().unwrap(),
            travel_dur,
            rest_dur,
            total_dur: travel_dur + rest_dur 
        });
    }

    reindeer
}
