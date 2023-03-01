#![allow(dead_code)]

use colored::Colorize;

pub fn show_prime(n: u64) {
	let mut i = 2;
	while i <= n {
		if is_prime(i) { 
			if is_prime(i - 2) || is_prime(i + 2) {
				print!("{}, ", i.to_string().blue().bold());
			} else {
				print!("{}, ", i);
			}
		}
		i += 1;
	}
}

fn is_prime(n: u64) -> bool {
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