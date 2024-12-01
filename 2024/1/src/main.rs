use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(left_column_values: &mut Vec<i32>, right_column_values: &mut Vec<i32>) {
	let file = File::open("src/input.txt")
		.expect("Should have been able to open");

	let reader = BufReader::new(file);

	for line in reader.lines() {
		if let Ok(line) = line {
			let numbers: Vec<&str> = line
				.split_whitespace()
				.collect();

			if numbers.len() >= 2 {
				if let Ok(left_num) = numbers[0].parse::<i32>() {
					left_column_values.push(left_num);
				}

				if let Ok(right_num) = numbers[1].parse::<i32>() {
					right_column_values.push(right_num);
				}
			}
		}
	}
}

fn summarize_comparison(left_column_values: Vec<i32>, right_column_values: Vec<i32>) -> i32 {
	let mut total = 0;

	for i in 0..left_column_values.len() {
		let distance = (left_column_values[i] - right_column_values[i]).abs();
		total += distance;
	}

	return total
}

fn main() {
	let mut left_column_values: Vec<i32> = Vec::new();
	let mut right_column_values: Vec<i32> = Vec::new();

	parse_input(&mut left_column_values, &mut right_column_values);
	left_column_values.sort_unstable();
	right_column_values.sort_unstable();

	let total_distance = summarize_comparison(left_column_values, right_column_values);

	println!("{:?}", total_distance);
}
