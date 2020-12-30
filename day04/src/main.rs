
fn main() {
    let key = "iwrupvqb";

    println!("Number for part 1: {}", calc_hash_number5(key));
    println!("Number for part 2: {}", calc_hash_number6(key));
}

fn calc_hash_number5(key: &str) -> u32 {
    for n in 0.. {
        let digest = md5::compute(format!("{}{}", key, n));
        if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0 {
            return n;
        }
    }

    0
}

fn calc_hash_number6(key: &str) -> u32 {
    for n in 0.. {
        let digest = md5::compute(format!("{}{}", key, n));
        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            return n;
        }
    }

    0
}

#[test]
fn test_md5() {
    let digest = md5::compute("abcdef609043");
    assert!(digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0);
}

#[test]
fn test_calc_hash_number5() {
    assert!(calc_hash_number5("abcdef") == 609043);
    assert!(calc_hash_number5("pqrstuv") == 1048970);
}
