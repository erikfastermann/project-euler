// always (L-n)*remaining_numbers
// 1: 1, 2: 1/2, 3: 2/3, 4: 1/2, 5: 4/5, 6: 1-(1/2+1/3) -> 1/6, 7: 6/7, 8: 1/2, ...

// examples:
// 5 * 2/3 == 3.333333333333333
// 3/8, 3/7, 3/5, 3/4
//
// 2 * 1/6 == 0.3333333333333333
// 6/7

use num_bigint::BigInt;
use num_rational::Ratio;

const L: u32 = 1000;

pub fn main() {
	let start = std::time::Instant::now();

	let mut found = BigInt::from(L-1);

	let mut factors = Vec::new();
	let one = Ratio::from(BigInt::from(1));
	for n in 2..=L {
		prime_factors(n, &mut factors);
		let mut remaining_numbers_factor = Ratio::from(BigInt::from(1));
		for factor in &factors {
			remaining_numbers_factor -= &one / BigInt::from(*factor);
		}
		found += (remaining_numbers_factor * BigInt::from(L-n)).ceil().to_integer();
		factors.clear();
	}
	dbg!(L, found, start.elapsed());
}

// https://www.geeksforgeeks.org/print-all-prime-factors-of-a-given-number/
fn prime_factors(mut n: u32, push_factors: &mut Vec<u32>) {
	assert!(n >= 2);

    if n%2 == 0 {
		push_factors.push(2);
		while n%2 == 0 {
			n /= 2;
		}
	}

    for i in (3..(f64::sqrt(n as f64) as u32) + 1).step_by(2) {
		if n%i == 0 {
			push_factors.push(i);
			while n%i == 0 {
				n /= i;
			}
		}
	}

    if n > 2 {
		push_factors.push(n);
	}
}