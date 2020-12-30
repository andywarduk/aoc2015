use std::{sync::{Arc, atomic::{AtomicUsize, Ordering}}, time::Instant};
use std::thread;

use md5::Digest;

type CheckFn = fn(Digest) -> bool;

fn main() {
    let key = "iwrupvqb";

    println!("Number for part 1: {}", time_exec(calc_hash_number, key, check5));
    println!("Number for part 2: {}", time_exec(calc_hash_number, key, check6));
    println!("Number for part 1 ({} threads): {}", threads(), time_exec(calc_hash_number_threaded, key, check5));
    println!("Number for part 2 ({} threads): {}", threads(), time_exec(calc_hash_number_threaded, key, check6));
}

fn time_exec(function: fn (key: &str, check: CheckFn) -> u32, key: &str, check: CheckFn) -> String {
    let now = Instant::now();

    let result = function(key, check);

    let elapsed = now.elapsed();

    format!("{} ({} secs)", result, elapsed.as_secs_f32())
}

fn threads() -> usize {
    num_cpus::get()
}

fn calc_hash_number(key: &str, check: CheckFn) -> u32 {
    for n in 0.. {
        let digest = md5::compute(format!("{}{}", key, n));
        if check(digest) {
            return n;
        }
    }

    0
}

fn check5(digest: Digest) -> bool{
    digest[0] == 0 && digest[1] == 0 && digest[2] & 0xf0 == 0
}

fn check6(digest: Digest) -> bool{
    digest[0] == 0 && digest[1] == 0 && digest[2] == 0
}

fn calc_hash_number_threaded(key: &str, check: fn(Digest) -> bool) -> u32 {
    let mut thread_handles = Vec::new();
    let result: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let tot_threads = threads();

    // Spawn threads
    for thread in 0..tot_threads {
        let thread_key = String::clone(&String::from(key));
        let thread_res = result.clone();

        let handle = thread::spawn(move || calc_hash_number_thread(thread, tot_threads, thread_key, thread_res, check));

        thread_handles.push(handle);
    }

    // Wait for threads
    for t in thread_handles {
        t.join().unwrap();
    }

    // Return result
    result.load(Ordering::Relaxed) as u32
}

fn calc_hash_number_thread(thread: usize, tot_threads: usize, key: String, res: Arc<AtomicUsize>, check: fn(Digest) -> bool) {
    for n in (thread..).step_by(tot_threads) {
        let digest = md5::compute(format!("{}{}", key, n));

        if check(digest) {
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
    assert!(calc_hash_number("abcdef", check5) == 609043);
    assert!(calc_hash_number("pqrstuv", check5) == 1048970);
}
