#![allow(non_snake_case)]
#![allow(dead_code)]

use colored::Colorize;

pub fn showPrime(n: u64) {
	let mut i = 2;
	while i <= n {
		if isPrime(i) { 
			if isPrime(i - 2) || isPrime(i + 2) {
				print!("{}, ", i.to_string().blue());
			} else {
				print!("{}, ", i);
			}
		}
		i += 1;
	}

}

fn isPrime(n: u64) -> bool {
	if n < 2 { return false; }
	if n <= 3 { return true; }
	if n % 2 == 0 || n % 3 == 0 { return false; }

	let mut i = 5;

	while i * i <= n {
		if n % i == 0 || n % (i + 2) == 0 { return false; };
		i += 6;
	}

	true
}