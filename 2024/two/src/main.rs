use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
	let parsed_reports: Vec<Vec<i32>> = parse_input();

	let safe_reports: i32 = safe_reports(parsed_reports);

	println!("1. Number of safe reports: {:?}", safe_reports);
}

fn parse_input() -> Vec<Vec<i32>> {
	let mut input = Vec::new();

	let file = File::open("src/input.txt")
		.expect("Should have been able to open");

	let reader = BufReader::new(file);

	for line in reader.lines() {
		if let Ok(line) = line {
			let numbers: Vec<i32> = line
				.split_whitespace()
				.map(|s| s.parse().unwrap())
				.collect();

			input.push(numbers);
		}
	}

	return input
}

fn is_monotonic(levels: Vec<i32>) -> bool {
	if levels.len() <= 1 {
		return true;
	}

	let increasing: bool = levels[1] > levels[0];

	for i in 1..levels.len() {
		if increasing {
			if levels[i] <= levels[i-1] {
				return false;
			}
		} else {
			if levels[i] >= levels[i-1] {
				return false;
			}
		}
	}

	true
}

fn is_within_threshold(levels: Vec<i32>) -> bool {
	const MIN_THRESHOLD: i32 = 1;
	const MAX_THRESHOLD: i32 = 3;

	if levels.len() <= 1 {
		return false;
	}

	for i in 1..levels.len() {
		let diff: i32 = (levels[i] - levels[i-1]).abs();

		if diff < MIN_THRESHOLD || diff > MAX_THRESHOLD {
			return false
		}
	}

	true
}

fn safe_reports(reports: Vec<Vec<i32>>) -> i32 {
	let mut safe_reports: i32 = 0;

	for i in 0..reports.len() {
		let levels: Vec<i32> = reports[i].clone();
		let levels_monotonic: bool = is_monotonic(levels.clone());

		if levels_monotonic {
			let levels_within_threshold: bool = is_within_threshold(levels);

			if levels_within_threshold {
				safe_reports += 1;
			}
		}
	}

	safe_reports
}
