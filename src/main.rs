#![allow(unused)]

use prime::show_prime;
use company::company;
use pig_latin::to_pig_latin;
use task1::{find_average, find_median, find_mode_of_list};

mod task1;
mod pig_latin;
mod prime;
mod company;

fn main() {
	// let mut s = "some interensting string one two three".to_string();
	// s = pig_latin::to_pig_latin(s).to_string();
	// print!("{s}");

	// show_prime(2000);

	// company();

	// let mut vec = vec![355,56,777,2000,357,1];

	// println!("{}", find_average(&vec));
	// println!("{}", find_median(&vec));
	// println!("{}", find_mode_of_list(&vec));
	// println!("{:?}", vec);
}
