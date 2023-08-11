use json::JsonValue;
use memmap::Mmap;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jsondoc = load_input("input12.txt")?;

    println!("Part 1 total: {}", walknode1(&jsondoc));
    println!("Part 2 total: {}", walknode2(&jsondoc));

    Ok(())
}

fn walknode1(node: &JsonValue) -> f64 {
    let mut total: f64 = 0.0;

    match node {
        JsonValue::Object(obj) => {
            for (_key, value) in obj.iter() {
                total += walknode1(value);
            }
        },
        JsonValue::Array(arr) => {
            for node in arr {
                total += walknode1(node);
            }            
        },
        JsonValue::Number(num) => {
            let rust_num: f64 = (*num).into();
            total += rust_num;
        },
        _ => {}
    }

    total
}

fn walknode2(node: &JsonValue) -> f64 {
    let mut total: f64 = 0.0;

    match node {
        JsonValue::Object(obj) => {
            let mut skip = false;

            for (_key, value) in obj.iter() {
                if value == "red" {
                    skip = true;
                    break
                }
            }

            if !skip {
                for (_key, value) in obj.iter() {
                    total += walknode2(value);
                }
            }
        },
        JsonValue::Array(arr) => {
            for node in arr {
                total += walknode2(node);
            }            
        },
        JsonValue::Number(num) => {
            let rust_num: f64 = (*num).into();
            total += rust_num;
        },
        _ => {}
    }

    total
}

fn load_input(file: &str) -> Result<JsonValue, Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(file)?;

    // Memory map it
    let mmap = unsafe { Mmap::map(&file)? };

    // Drop the file
    drop(file);

    // Convert mmap to str
    let content = unsafe { std::mem::transmute::<Mmap, &str>(mmap) };

    // Parse the JSON
    let parsed = json::parse(content)?;

    Ok(parsed)
}
