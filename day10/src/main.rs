fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut string = "1113222113".to_string();

    for _ in 0..40 {
        string = transform(string);
    }

    println!("Length after 40 iterations (part 1): {}", string.len());

    for _ in 0..10 {
        string = transform(string);
    }

    println!("Length after 50 iterations (part 1): {}", string.len());

    Ok(())
}

fn transform(string: String) -> String {
    let chars: Vec<char> = string.chars().collect();
    let mut last = chars[0];
    let mut count: usize = 1;
    let mut out_chars = Vec::new();

    let mut add = |char, count: usize| {
        let mut count_chars: Vec<char> = count.to_string().chars().collect();
        out_chars.append(&mut count_chars);
        out_chars.push(char);
    };

    for i in 1..chars.len() {
        if chars[i] != last {
            add(last, count);
            count = 0;
            last = chars[i];
        }
        count += 1;
    }
    add(last, count);

    out_chars.iter().collect()
}
