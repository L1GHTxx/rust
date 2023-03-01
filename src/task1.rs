#![allow(dead_code)]

use std::collections::HashMap;

pub fn find_average(vec: Vec<i32>) -> f32 {
	if vec.len() == 0 {
		println!("the length of the vector is zero");
		return 0.0;
	}

	let sum: i32 = vec.iter().sum();
	let average: f32 = sum as f32 / vec.len() as f32;
	
	average
}

pub fn find_median(vec: &mut Vec<i32>) -> i32 {
	if vec.len() == 0 {
		println!("the length of the vector is zero");
		return 0;
	}

	vec.sort();
	let median: i32 = vec[vec.len() / 2];

	median
}

pub fn find_mode_of_list(vec: Vec<i32>) -> i32 {
	let mut map = HashMap::new();
	let len = vec.len() as usize;

	for s in 0..len {
		let i = map.entry(vec[s]).or_insert(0);
		*i += 1;
	}

	let mut mode_of_list: i32 = vec[0];
	let mut max = 0;
	for c in map {
		if c.1 > max {
			max = c.1;
			mode_of_list = c.0;
		}
	};

	mode_of_list
}