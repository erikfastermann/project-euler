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

pub fn main() {
	let args: Vec<_> = std::env::args().collect();
	let l = args[1].parse::<u32>().unwrap();

	let start = std::time::Instant::now();
	let mut found = BigInt::from(l-1);

	let one = Ratio::from(BigInt::from(1));
	for n in 2..=l {
		let factors = primes::factors_uniq(n.into());
		let mut remaining_numbers_factor = Ratio::from(BigInt::from(1));
		for factor in &factors {
			remaining_numbers_factor -= &one / BigInt::from(*factor);
		}
		found += (remaining_numbers_factor * BigInt::from(l-n)).ceil().to_integer();
	}

	dbg!(l, found, start.elapsed());
	println!("{}", found);
}

// 2: slow=1 fast=1
// 3: slow=3 fast=3
// 4: slow=5 fast=5
// 5: slow=9 fast=9
// 6: slow=11 fast=11
// 7: slow=17 fast=17
// 8: slow=21 fast=21
// 9: slow=27 fast=27
// 10: slow=31 fast=31
// 11: slow=41 fast=40
// 12: slow=45 fast=44
// 13: slow=57 fast=55
// 14: slow=63 fast=62
// 15: slow=71 fast=70
// 16: slow=79 fast=78
// 17: slow=95 fast=91
// 18: slow=101 fast=98
// 19: slow=119 fast=113
// 20: slow=127 fast=123
// 21: slow=139 fast=135
// 22: slow=149 fast=145
// 23: slow=171 fast=161
// 24: slow=179 fast=172
// 25: slow=199 fast=191
// 26: slow=211 fast=203
// 27: slow=229 fast=221
// 28: slow=241 fast=233
// 29: slow=269 fast=253
// 30: slow=277 fast=262