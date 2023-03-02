#![allow(unused)]

use company::company;
use pig_latin::convert_to_pig_latin;
use prime::show_prime;
use task1::{find_average, find_median, find_mode_of_list};
use volt_divider_calc::volt_divider_calc;

mod company;
mod pig_latin;
mod prime;
mod task1;
mod volt_divider_calc;

fn main() {
    // let mut s = "some interensting string one two three".to_string();
    // s = convert_pig_latin::to_pig_latin(s).to_string();
    // print!("{s}");

    // show_prime(1000);

    // company();

     let mut vec = vec![355,56,777,2000,357];

    // println!("{}", find_average(&vec));
     println!("{}", find_median(&vec));
    // println!("{}", find_mode_of_list(&vec));
	vec.sort();
     println!("{:?}", vec);

    
}
