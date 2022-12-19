use rand::{thread_rng, Rng};

fn modsquare(x: u64, modulus: u64) -> u64 {
	let x : u128 = x as u128;
	(x * x % (modulus as u128)) as u64
}

fn modpow(base: u64, mut exponent: u64, modulus: u64) -> u64 {
	if exponent == 0 {
		return 1;
	}
 	let mut result : u128 = 1;
	let modulus : u128 = modulus as u128;
	let mut base : u128 = base as u128;
	while exponent > 1 {
		if (exponent & 1) != 0 {
			result = result * base % modulus;
		}
		base = base * base % modulus;
		exponent >>= 1;
	}
	(result * base % modulus) as u64
}

fn factor(mut num: u64) -> Vec<(u64, u64)> {
	let mut factors: Vec<(u64, u64)> = vec![];
	if num <= 1 {
		return factors;
	}

	let mut power: u64 = num.trailing_zeros() as u64;
	if power > 0 {
		num >>= power;
		factors.push((2, power));
		power = 0;
	}

	while (num % 3) == 0 {
		num /= 3;
		power += 1;
	}
	if power > 0 {
		factors.push((3, power));
		power = 0;
	}

	while (num % 5) == 0 {
		num /= 5;
		power += 1;
	}
	if power > 0 {
		factors.push((5, power));
		power = 0;
	}

	let increments =  [4, 2, 4, 2, 4, 6, 2, 6];
	let mut divisor: u64 = 7; //11 13 17 19 23 29 31 37
	let mut upper_bound: u64 = (num as f64).sqrt() as u64;
	let mut index = 0;

	while divisor <= upper_bound {
		while (num % divisor) == 0 {
			num /= divisor;
			power += 1;
		}
		if power > 0 {
			factors.push((divisor, power));
			upper_bound = (num as f64).sqrt() as u64;
			power = 0;
		}
		divisor += increments[index & 7];
		index += 1;
	}
	if num > 1 {
		factors.push((num, 1));
	}
	factors
}

#[allow(dead_code)]
fn gcd(a: u64, b: u64 ) -> u64 {
	if b == 0 {
		a
	} else {
		gcd(b, a % b)
	}
}

fn miller_rabin(number: u64) -> bool {
	let s = (number - 1).trailing_zeros();
	let d = number >> s;
	'outer: for witness in [ 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37 ] {
		/* if gcd(witness, number) != 1 {
			return witness == number; // could just return false if we know number > witness
		} */
		if number % witness == 0 { // this is equivalent and faster since witnesses are all primes
			return number == witness;
		}
		let mut x : u64 = modpow(witness, d, number);
		if x == 1 || x == number - 1 {
			continue;
		}
		let mut i = s - 1;
		while i > 0 {
			x = modsquare(x, number);
			if x == number - 1 {
				continue 'outer;
			}
			i -= 1;
		}
		return false;
	}
	return true;
}

#[allow(dead_code)]
fn rand_prime_between(min: u64, max: u64) -> Option<u64> {
	let mut rng = thread_rng();
	let mut primes = vec![];
	for n in min..=max {
		if miller_rabin(n) {
			primes.push(n);
		}
	}
	if primes.is_empty() {
		return None;
	}
	Some(primes[rng.gen_range(0..primes.len())])
}

#[allow(dead_code)]
fn next_prime(value: u64, dir: Option<i32>) -> u64 {
	let mut direction = match dir {
		Some(0) => 1,
		Some(i) => i,
		None => 1
	};
	if direction >= 1 {
		let mut number = value + 1 | 1;
		while direction != 0 {
			if miller_rabin(number) {
				direction -= 1;
			}
			number += 2;
		}
		number
	} else {
		let mut number = value - 2 | 1;
		while direction != 0 {
			if miller_rabin(number) {
				direction += 1;
			}
			number -= 2;
		}
		number
	}
}

#[allow(dead_code)]
fn rand_normal() {
	let div = 1.0 / (u64::MAX as f64);
	let scale = 0.24743582965269675;
	let mut rng = thread_rng();
	let x = rng.gen::<i64>();
	let popcnt: i32 = x.count_ones() - 32;
	(popcnt + (x as f64) * div) * scale
} 

fn main() { // generates a prime number between 2^63 and 2^64 - 1
	let mut rng = thread_rng();
	loop {
		let n: u64 = rng.gen::<u64>() | 1 << 63 | 1;
		if miller_rabin(n) {
			println!("{n} is prime: {}", factor(n)[0].0);
			return;
		}
		print!("{n} is not prime: ");
		let factors = factor(n);
		print!("{}", factors[0].0);
		if factors[0].1 > 1 {
			print!("^{}", factors[0].1);
		}
		for (base, exponent) in factors.iter().skip(1) {
			print!(" * {base}");
			if exponent > &1 {
				print!("^{exponent}");
			}
		}
		println!();
	}
}

#[cfg(test)]
mod test {
	use crate::*;
	#[test]
	fn test_modsquare() {
		assert_eq!(modsquare(47, 99), 31);
		assert_eq!(modsquare(18446744073709551557u64, 18446744073709551533u64), 576);
	}

	#[test]
	fn test_modpow() {
		assert_eq!(modpow(2, 10, 10), 4);
		assert_eq!(modpow(18446744073709551557u64, 18446744073709551533u64, 18446744073709551521u64), 4561031516192244567u64);
	}

	#[test]
	fn test_factor() {
		assert_eq!(factor(1u64 << 53), vec![(2, 53)]);
		assert_eq!(factor(47u64 << 53), vec![(2, 53), (47, 1)]);
		assert_eq!(factor((1u64 << 53) - 111), vec![(((1u64 << 53) - 111), 1)]);
		assert_eq!(factor(4294967291u64 * 4294967279u64), vec![(4294967279, 1), (4294967291, 1)]);
	}

	#[test]
	fn test_gcd() {
		assert_eq!(gcd(100, 101), 1);
		assert_eq!(gcd(610, 377), 1);
		assert_eq!(gcd(341550071728321, 32010157 * 66670053), 32010157);
		assert_eq!(gcd(modpow(30, 13, u64::MAX), modpow(105, 9, u64::MAX)), modpow(15, 9, u64::MAX));
	}

	#[test]
	fn test_mr() {
		assert_eq!(miller_rabin((1u64 << 53) - 111), true);
		assert_eq!(miller_rabin(31), true);
		assert_eq!(miller_rabin(961), false);
		assert_eq!(miller_rabin((1u64 << 53) - 1), false);
		assert_eq!(miller_rabin(341550071728321), false);
	}
}