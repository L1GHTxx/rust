#![allow(dead_code)]

use std::collections::HashMap;
use std::io::stdin;

pub fn company() {	//название временное
	let mut map: HashMap<String, Vec<String>> = HashMap::new();
	loop {
		let mut choice = String::new();

		println!("\n1 - add | 2 - show | any - exit ");
		stdin().read_line(&mut choice).expect("input error");

		match choice.trim().parse() {
			Ok(1) => {
				add_employee(&mut map);
			},
			Ok(2) => {
				show(&mut map);
			},
			_ =>  return,
		};
	}
}

fn add_employee(map: &mut HashMap<String, Vec<String>>) {
	let mut department = String::new();
	let mut names: Vec<String> = Vec::new();

	println!("Enter department: ");
	stdin().read_line(&mut department).expect("input error");

	department = department.trim().to_string();

	println!("Enter a names through the enter key or \"stop\" to stop entering: ");
	loop {
		let mut name = String::new();

		stdin().read_line(&mut name).expect("input error");

		match name.as_str() {
		"stop\r\n" => break,
		"\r\n" => println!("\"Enter\" isn\'t name.\n"),
		_ => { 
			name = name.trim().to_string(); 
			names.push(name); 
		},
		}
	};
	
	map.insert(department, names);
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