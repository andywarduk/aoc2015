use memmap::Mmap;
use std::fs::File;
use std::str;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let directions = load_input("input01.txt")?;

    let mut floor = 0;

    for d in directions.chars() {
        match d {
            '(' => floor +=1,
            ')' => floor -=1,
            _ => {}
        }
    }

    println!("{}", floor);

    Ok(())
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
