use num_rational::Ratio;

fn prime_factors(mut n: u32, primes: &mut Vec<u32>) {
    // adapted from
    // https://www.geeksforgeeks.org/print-all-prime-factors-of-a-given-number/

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
}

fn phi(n: u32, primes: &mut Vec<u32>) -> u32 {
    let mut res = Ratio::from(n);
    prime_factors(n, primes);
    for p in primes {
        res *= Ratio::from(1) - Ratio::new(1, *p);
    }
    assert_eq!(*res.denom(), 1);
    res.to_integer()
}

fn digit_count(mut n: u32, counts: &mut [usize; 10]) {
    while n != 0 {
        counts[(n%10) as usize] += 1;
        n /= 10;
    }
}

fn is_permutation_base_10(x: u32, y: u32) -> bool {
    let mut x_count = [0; 10];
    let mut y_count = [0; 10];
    digit_count(x, &mut x_count);
    digit_count(y, &mut y_count);
    x_count == y_count
}

fn calc() -> u32 {
    let mut found_n = 0;
    let mut found_min = Ratio::from(u32::MAX);

    let mut primes_cache = Vec::new();
    for n in 2..10u32.pow(7) {
        if n%1_000_000 == 0 {
            dbg!(n);
        }

        let phi_n = phi(n, &mut primes_cache);
        if is_permutation_base_10(n, phi_n) {
            let n_phi_n = Ratio::new(n, phi_n);
            if &n_phi_n < &found_min {
                found_n = n;
                found_min = n_phi_n;
            }
        }

        primes_cache.clear();
    }

    found_n
}

fn main() {
    let start = std::time::Instant::now();
    println!("{} ({:?})", calc(), start.elapsed());
}