use num_rational::Ratio;
use std::collections::HashMap;

fn prime_factors(mut n: u32) -> Vec<u32> {
    // adapted from
    // https://www.geeksforgeeks.org/print-all-prime-factors-of-a-given-number/

    let mut primes = Vec::new();

    if n%2 == 0 {
        primes.push(2);
        while n%2 == 0 {
            n /= 2;
        }
    }

    let to = f64::sqrt(n.try_into().unwrap()) as u32 + 1;
    for i in (3..to).step_by(2) {
        if n%i == 0 {
            primes.push(i);
            while n%i == 0 {
                n /= i;
            }
        }
    }

    if n > 2 {
        primes.push(n);
    }

    primes
}

fn phi(n: u32) -> u32 {
    let mut res = Ratio::from(n);
    for p in prime_factors(n) {
        res *= Ratio::from(1) - Ratio::new(1, p);
    }
    assert_eq!(*res.denom(), 1);
    res.to_integer()
}

fn digit_count(x: u32) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for digit in x.to_string().chars() {
        *counts.entry(digit).or_insert(0) += 1;
    }
    counts
}

fn is_permutation_base_10(x: u32, y: u32) -> bool {
    digit_count(x) == digit_count(y)
}

fn calc() -> u32 {
    let mut found_n = 0;
    let mut found_min = Ratio::from(u32::MAX);

    for n in 2..10u32.pow(7) {
        if n%1_000_000 == 0 {
            dbg!(n);
        }

        let phi_n = phi(n);
        if is_permutation_base_10(n, phi_n) {
            let n_phi_n = Ratio::new(n, phi_n);
            if &n_phi_n < &found_min {
                found_n = n;
                found_min = n_phi_n;
            }
        }
    }

    found_n
}

fn main() {
    let start = std::time::Instant::now();
    println!("{} ({:?})", calc(), start.elapsed());
}