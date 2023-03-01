#![allow(dead_code)]

pub fn to_pig_latin(s: String) -> String {

	let mut new_str = "".to_string();

	let vowel = ["a", "e", "i", "o", "u", "y"];

    for word in s.split_whitespace() {
        let mut is_first_vowel: bool = false;
		for i in 0..vowel.len() {
			if &word[..1].to_lowercase() == &vowel[i].to_lowercase() {
				is_first_vowel = true;
			}
		};

		if is_first_vowel {
			new_str = new_str + word + "-hay ";
		} else {
			let mut new_word: String = word.to_string() + "-" + &word[..1] + "ay ";
			new_word.remove(0);
			new_str = new_str + &new_word;
		}
    }
	new_str
}