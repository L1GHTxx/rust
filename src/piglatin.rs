#![allow(dead_code)]

pub fn piglatin(s: String) -> String {

	let mut new_s = "".to_string();

	let gl = ["a", "e", "i", "o", "u", "y"];

    for word in s.split_whitespace() {
        let mut is_first_gl: bool = false;
		for i in 0..gl.len() {
			if &word[..1].to_lowercase() == &gl[i].to_lowercase() {
				is_first_gl = true;
			}
		};

		if is_first_gl {
			new_s = new_s + word + "-hay ";
		} else {
			let mut new_word: String = word.to_string() + "-" + &word[..1] + "ay ";
			new_word.remove(0);
			new_s = new_s + &new_word;
		}
    }
	new_s
}