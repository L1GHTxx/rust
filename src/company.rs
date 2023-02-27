#![allow(dead_code)]

use std::collections::HashMap;
use std::io::*;

pub fn company() {
	let mut map: HashMap<String, Vec<String>> = HashMap::new();
	loop {
		let mut choice = String::new();

		println!("\n1 - add | 2 - show | any - exit ");
		stdin().read_line(&mut choice).expect("⚠ - znachec chto ti duracheck");

		match choice.as_str() {
			"1\r\n" => {
				add(&mut map);
			},
			"2\r\n" => {
				show(&mut map);
			},
			_ =>  return,
		};
	}

	add(&mut map);
}

fn add(map: &mut HashMap<String, Vec<String>>) {
	let mut key = String::new();
	let mut names: Vec<String> = Vec::new();

	println!("Enter department: ");
	stdin().read_line(&mut key).expect("⚠ - znachec chto ti duracheck");

	key.pop();
	key.pop();

	loop {
		let mut name = String::new();
		println!("Enter name or \"stop\" to stop entering: ");
		stdin().read_line(&mut name).expect("⚠ - znachec chto ti duracheck");

		match name.as_str() {
		"stop\r\n" => break,
		"\r\n" => println!("Enter isn\'t name.\n"),
		_ => { name.pop(); name.pop(); names.push(name); },
		}
	};
	
	map.insert(key, names);
}

fn show(map: &mut HashMap<String, Vec<String>>) {
	if map.is_empty() { println!("List is empty."); return; }

	for c in map {
		println!("\n{} includes:", c.0);
		if c.1.is_empty() {
			println!("NO ONE");
		} else {
			c.1.sort();
			for c in c.1 {
				println!("{c}");
			}
		}
	}
}