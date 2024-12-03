use regex::Regex;
use std::fs;

struct Multiplication {
    x: i32,
    y: i32,
}

impl Multiplication {
	fn result(&self) -> i32 {
		self.x * self.y
	}
}

fn main() {
	let file: String = read_file();
	let uncorrupt_muls: Vec<Multiplication> = find_uncorrupt_mul(&file);
	let summarized_muls: i32 = summarize_uncorrupt_muls(uncorrupt_muls);

	println!("1. Summarized uncorrupt muls: {:?}", summarized_muls);
}

fn read_file() -> String {
	let file_string = fs::read_to_string("src/input.txt")
		.expect("Should be able to open file");

	file_string
}

fn find_uncorrupt_mul(file: &str) -> Vec<Multiplication> {
	let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

	re.captures_iter(file)
		.map(|cap| Multiplication {
			x: cap[1].parse().unwrap(),
			y: cap[2].parse().unwrap()
		})
		.collect()
}

fn summarize_uncorrupt_muls(uncorrupt_muls: Vec<Multiplication>) -> i32 {
	let mut sum_muls: i32 = 0;

	for mul in uncorrupt_muls {
		sum_muls += mul.result();
	}

	sum_muls
}
