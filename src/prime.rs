#![allow(dead_code)]

use colored::Colorize;

pub fn show_prime(n: u64) {
    let mut i = 3;
    while i <= n {
        if is_prime(i) {
            if is_prime(i + 2) {
                print!("{}, {}, ", i.to_string().blue(), (i + 2).to_string().blue());
				i += 2;
            } else {
                print!("{}, ", i);
            }
        }
		i += 2;
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;

    while i as f32  <= (n as f32).sqrt() {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        };
        i += 6;
    }

    return true;
}
