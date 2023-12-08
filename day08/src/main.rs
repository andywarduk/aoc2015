use memmap2::Mmap;
use std::{fs::File, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = load_input("input08.txt")?;

    part1(&lines);

    part2(&lines);

    Ok(())
}

fn part1(lines: &[String]) {
    let sum: usize = lines.iter().map(decode_diff).sum();

    println!("Length difference is {} (part 1)", sum);
}

fn part2(lines: &[String]) {
    let sum: usize = lines.iter().map(encode_diff).sum();

    println!("Length difference is {} (part 2)", sum);
}

fn decode_diff(string: &String) -> usize {
    string.len() - count_decode_chars(string)
}

fn count_decode_chars(string: &String) -> usize {
    let mut c = 1;
    let mut count = 0;

    let chars: Vec<char> = string.chars().collect();

    assert!(chars[0] == '"');
    assert!(chars[string.len() - 1] == '"');

    while c < string.len() - 1 {
        c += match chars[c] {
            '\\' => {
                match chars[c + 1] {
                    '\\' | '"' => 2,
                    'x' => 4,
                    _ => panic!("Unrecognised escape")
                }
            },
            _ => 1
        };

        count += 1
    }

    count
}

fn encode_diff(string: &String) -> usize {
    count_encode_chars(string) - string.len()
}

fn count_encode_chars(string: &str) -> usize {
    let mut count = 0;

    for c in string.chars() {
        count += match c {
            '\\' | '"' => 2,
            _ => 1
        }
    }

    2 + count
}

#[test]
fn test_count_decode_chars() {
    assert!(count_decode_chars(&"\"\"".to_string()) == 0);
    assert!(count_decode_chars(&"\"abc\"".to_string()) == 3);
    assert!(count_decode_chars(&"\"aaa\\\"aaa\"".to_string()) == 7);
    assert!(count_decode_chars(&"\"\\x27\"".to_string()) == 1);
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
