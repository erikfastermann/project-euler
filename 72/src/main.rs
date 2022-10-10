use num_rational::Ratio;

fn totient(d: u64) -> u64 {
	let one = Ratio::from(1u64);
	let factors = primes::factors_uniq(d.into());
	factors.iter().fold(Ratio::from(d),
		|acc, p| acc * (&one - (&one / *p))).to_integer()
}

fn totient_sum(l: u64) -> u64 {
	let mut sum = 0u64;
	for d in 2..=l {
		sum += totient(d);
	}
	sum
}

pub fn main() {
	let args: Vec<_> = std::env::args().collect();
	let l = args[1].parse::<u64>().unwrap();

	let start = std::time::Instant::now();
	let found = totient_sum(l);
	dbg!(l, &found, start.elapsed());
	println!("{}", &found);
}