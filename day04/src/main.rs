use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;

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

const THREADS: usize = 8;

fn calc_hash_number6(key: &str) -> u32 {
    let mut threads = Vec::new();
    let result: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

    // Spawn threads
    for thread in 0..THREADS {
        let thread_key = String::clone(&String::from(key));
        let thread_res = result.clone();

        let handle = thread::spawn(move || calc_hash_number6_thread(thread, thread_key, thread_res));

        threads.push(handle);
    }

    // Wait for threads
    for t in threads {
        t.join().unwrap();
    }

    // Return result
    result.load(Ordering::Relaxed) as u32
}

fn calc_hash_number6_thread(thread: usize, key: String, res: Arc<AtomicUsize>) {
    for n in (thread..).step_by(THREADS) {
        let digest = md5::compute(format!("{}{}", key, n));

        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            // Found an answer
            let mut result = res.load(Ordering::Acquire);

            if result == 0 || result > n {
                // Set the atomic usize to our answer
                result = n
            }

            res.store(result, Ordering::Release);

            break;
        }

        let result = res.load(Ordering::Relaxed);

        if result != 0 && result < n {
            // Another thread found an answer
            break
        }
    }
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
