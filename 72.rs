// rustc -O -C target-cpu=native 72.rs && ./72

const L: u32 = 1_000_000;

fn hcf_is_one(n: u32, d: u32) -> bool {
	if n%2 == 0 && d%2 == 0 {
		return false;
	}

	// https://www.cuemath.com/numbers/hcf-highest-common-factor/
	// HCF by Division Method

	// d is always larger than n
	let mut dividend = d;
	let mut divisor = n;
	loop {
		let remainder = dividend % divisor;
		if remainder == 0 {
			break divisor == 1;
		}
		(dividend, divisor) = (divisor, remainder);
	}
}

pub fn main() {
	let start = std::time::Instant::now();
	let mut found = 0;
	for d in 2..=L {
        if d%10_000 == 0 {
            dbg!(d);
        }

		for n in 1..d {
			if hcf_is_one(n, d) {
                // println!("{} {}", n, d);
				found += 1;
			}
		}
	}
	dbg!(L, found, start.elapsed());
}
