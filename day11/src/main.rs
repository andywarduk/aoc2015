fn main() {
    let mut pwd: String = "hepxcrrq".to_string();

    pwd = calc_next_password(pwd);
    println!("Next password for part 1: {}", pwd);

    pwd = calc_next_password(pwd);
    println!("Next password for part 2: {}", pwd);
}

const ALPHABET: &str = "abcdefghjkmnpqrstuvwxyz";

fn calc_next_password(pwd: String) -> String {
    // Convert to numbers
    let mut pwdnum: Vec<u8> = pwd.chars().map(|c| ALPHABET.find(c).unwrap() as u8).collect();

    loop {
        // Increment the password
        increment_pwdnum(&mut pwdnum);

        // Validate it
        if validate_pwdnum(&pwdnum) {
            break
        }
    }

    pwdnum.iter().map(|n| ALPHABET.chars().nth(*n as usize).unwrap()).collect()
}

fn increment_pwdnum(pwdnum: &mut Vec<u8>) {
    let charcnt = pwdnum.len();
    let alen = ALPHABET.len() as u8;

    let mut curchar = charcnt - 1;
    loop {
        let newval = pwdnum[curchar] + 1;

        if newval < alen {
            pwdnum[curchar] = newval;
            break
        }

        pwdnum[curchar] = 0;
        curchar -= 1;
    }
}

fn validate_pwdnum(pwdnum: &Vec<u8>) -> bool {
    let charcnt = pwdnum.len();

    let mut got_triple = false;

    for i in 0..charcnt - 2 {
        if pwdnum[i + 1] == pwdnum[i] + 1 && pwdnum[i + 2] == pwdnum[i] + 2 {
            got_triple = true;
            break
        }
    }

    if !got_triple {
        return false
    }

    let mut got_pair1 = false;
    let mut pair1_pos = 0;
    let mut pair1_char = 0;

    for i in 0..charcnt - 1 {
        if pwdnum[i] == pwdnum[i + 1] {
            got_pair1 = true;
            pair1_pos = i;
            pair1_char = pwdnum[i];
            break
        }
    }

    if !got_pair1 {
        return false
    }

    let mut got_pair2 = false;

    for i in pair1_pos..charcnt - 1 {
        if pwdnum[i] == pwdnum[i + 1] && pwdnum[i] != pair1_char {
            got_pair2 = true;
            break
        }
    }

    if !got_pair2 {
        return false
    }

    true
}

#[test]
fn test_calc_next_password() {
    assert!(calc_next_password("abcdefgh".to_string()) == "abcdffaa");
    assert!(calc_next_password("ghijklmn".to_string()) == "ghjaabcc");
}