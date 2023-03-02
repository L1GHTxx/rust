#![allow(dead_code)]

use std::collections::HashMap;

pub fn find_average(vec: &Vec<i32>) -> f32 {
    if vec.len() == 0 {
        println!("the length of the vector is zero");
        return 0.0;
    }

	return vec.iter().sum::<i32>() as f32 / vec.len() as f32;
}

pub fn find_median(vec: &Vec<i32>) -> i32 {
    if vec.len() == 0 {
        println!("the length of the vector is zero");
        return 0;
    }

    let mut temp_vec = vec.clone();
	let vec_len = temp_vec.len();
	let median: i32;
    temp_vec.sort();

	if vec_len % 2 == 0 {
		median = (temp_vec[vec_len / 2] + temp_vec[vec_len / 2  - 1]) / 2;
	} else {
		median = temp_vec[vec_len / 2];
	}

    return median;
}

pub fn find_mode_of_list(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let len = vec.len() as usize;

    for i in 0..len {
        let i = map.entry(vec[i]).or_insert(0);
        *i += 1;
    }

    let mut mode_of_list: i32 = vec[0];
    let mut max = 0;
    for c in map {
        if c.1 > max {
            max = c.1;
            mode_of_list = c.0;
        }
    }

    return mode_of_list;
}
