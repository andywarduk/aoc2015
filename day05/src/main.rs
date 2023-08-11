use memmap::Mmap;
use std::{fs::File, io::{BufRead, BufReader}};
use std::str;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input05.txt")?;

    // Part 1
    let category: Vec<LineStatus> = lines.iter().map(|s| analyse_line1(s)).collect();

    println!("Nice lines (part 1): {}", category.iter().filter(|&c| *c == LineStatus::Nice).count());

    // Part 2
    let category: Vec<LineStatus> = lines.iter().map(|s| analyse_line2(s)).collect();

    println!("Nice lines (part 2): {}", category.iter().filter(|&c| *c == LineStatus::Nice).count());

    Ok(())
}

#[derive(PartialEq, Eq)]
enum LineStatus {
    Naughty,
    Nice
}

fn analyse_line1(line: &str) -> LineStatus {
    let mut vowel_count = 0;

    for c in line.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
            _ => {}
        }
    }

    if vowel_count < 3 {
        return  LineStatus::Naughty
    }

    let mut double = false;

    for i in 1..line.len() {
        if line.as_bytes()[i - 1] == line.as_bytes()[i] {
            double = true;
            break
        }
    }

    if !double {
        return LineStatus::Naughty
    }

    for i in 1..line.len() {
        match line.as_bytes()[i - 1] {
            b'a' => {
                if line.as_bytes()[i] == b'b' { return LineStatus::Naughty }
            },
            b'c' => {
                if line.as_bytes()[i] == b'd' { return LineStatus::Naughty }
            },
            b'p' => {
                if line.as_bytes()[i] == b'q' { return LineStatus::Naughty }
            },
            b'x' => {
                if line.as_bytes()[i] == b'y' { return LineStatus::Naughty }
            },
            _ => {}
        }
    }

    LineStatus::Nice
}

fn analyse_line2(line: &str) -> LineStatus {
    let chars: Vec<char> = line.chars().collect();

    let mut found = false;

    for i in 1..chars.len() - 2 {
        for j in i + 2..chars.len() {
            if chars[i - 1] == chars[j - 1] && chars[i] == chars[j] {
                // Found two doubles
                found = true;
                break;
            }
        }
    }

    if !found {
        return LineStatus::Naughty;
    }

    found = false;

    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            // Found a pair with 1 character gap
            found = true;
            break;
        }
    }

    if !found {
        return LineStatus::Naughty;
    }

    LineStatus::Nice
}

#[test]
fn test_analyse_line2() {
    assert!(analyse_line2("qjhvhtzxzqqjkmpb") == LineStatus::Nice);
    assert!(analyse_line2("xxyxx") == LineStatus::Nice);
    assert!(analyse_line2("xxxxaba") == LineStatus::Nice);
    assert!(analyse_line2("abaxxxx") == LineStatus::Nice);
    assert!(analyse_line2("tdfvkreormspprer") == LineStatus::Nice);
    assert!(analyse_line2("xxxaba") == LineStatus::Naughty);
    assert!(analyse_line2("abaxxx") == LineStatus::Naughty);
    assert!(analyse_line2("abcxxxcbd") == LineStatus::Naughty);
    assert!(analyse_line2("uurcxstgmygtbstg") == LineStatus::Naughty);
    assert!(analyse_line2("ieodomkazucvgmuy") == LineStatus::Naughty);
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
