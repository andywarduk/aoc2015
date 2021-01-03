const PRESENT_THRESH: u64 = 29_000_000;

fn main() {
    let mut primes = Vec::with_capacity(1024);

    for i in 1.. {
        let sum = prime_fact_sum(i, &mut primes);

        let presents = sum * 10;

        if presents > PRESENT_THRESH {
            println!("{} = {} -> {}", i, sum, sum * 10);
            println!("House: {}", i);
            break
        }
    }
}

fn prime_fact_sum(num: u64, primes: &mut Vec<u64>) -> u64 {
    let mut remain = num;
    let mut pelem = 0;
    let mut total: u64 = 1;

    let mut add_power = |num: u64, power: u32| {
        let power_sum = (num.pow(power + 1) - 1) / (num - 1);
        total *= power_sum;
    };

    while pelem < primes.len() {
        let prime = primes[pelem];

        let mut power = 0;

        while remain > 1 && remain % prime == 0 {
            remain /= prime;
            power += 1;
        }

        if power > 0 {
            add_power(prime, power);
        }

        if remain == 1 {
            break;
        }

        pelem += 1;
    }

    if remain != 1 {
        primes.push(remain);
        add_power(remain, 1);
    }

    total
}
