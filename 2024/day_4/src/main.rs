use::std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct TraversalPath {
	lines: Vec<Vec<char>>
}

impl TraversalPath {
	fn new_adventure(path_to_moria: &str) -> Self {
		let doors_of_durin = File::open(path_to_moria)
			.expect("Expected Mellon, got dirty chants from Gandalf");

		let lines = BufReader::new(doors_of_durin)
			.lines()
			.map(|line| line.unwrap().chars().collect())
			.collect();

		TraversalPath { lines }
	}

	fn horizontal(&self) -> Vec<Vec<char>> {
		self.lines.clone()
	}

	fn horizontal_backwards(&self) -> Vec<Vec<char>> {
		self.lines.iter()
			.map(|line| line.iter().rev().cloned().collect())
			.collect()
	}

	fn vertical(&self) -> Vec<Vec<char>> {
		if self.lines.is_empty() {
						return vec![];
		}

		let width = self.lines[0].len();
		let height = self.lines.len();
		let mut vertical = vec![vec![' '; height]; width];

		for (i, row) in self.lines.iter().enumerate() {
			for (j, &ch) in row.iter().enumerate() {
				vertical[j][i] = ch;
			}
		}

		vertical
	}

	fn vertical_backwards(&self) -> Vec<Vec<char>> {
		self.vertical()
			.iter()
			.map(|line| line.iter().rev().cloned().collect())
			.collect()
	}

	fn diagonal(&self) -> Vec<Vec<char>> {
		if self.lines.is_empty() {
			return vec![];
		}

		let mut diagonals = Vec::new();
		let width = self.lines[0].len();
		let height = self.lines.len();

		let mut collect_diagonal = |start_row: usize, start_col: usize, step: i32| {
			let mut diagonal = Vec::new();
			let mut row = start_row;
			let mut col = start_col;

			while row < height && (step > 0 && col < width || step < 0 && col < width) {
				diagonal.push(self.lines[row][col]);
				row += 1;
				col = (col as i32 + step) as usize;
			}

			if diagonal.len() > 1 {
				diagonals.push(diagonal);
			}
		};

		for start_col in 0..width {
			collect_diagonal(0, start_col, 1);
		}
		for start_row in 1..height {
			collect_diagonal(start_row, 0, 1);
		}

		for start_col in 0..width {
			collect_diagonal(0, start_col, -1);
		}
		for start_row in 1..height {
			collect_diagonal(start_row, width - 1, -1);
		}

		diagonals
	}

	fn diagonal_backwards(&self) -> Vec<Vec<char>> {
		self.diagonal()
			.iter()
			.map(|diagonal| diagonal.iter().rev().cloned().collect())
			.collect()
	}
}

fn main() {
	let path_to_moria = TraversalPath::new_adventure("src/input.txt");
	let mut xmas_count = 0;
	let mut all_traversals = Vec::new();

	all_traversals.extend(path_to_moria.horizontal());
	all_traversals.extend(path_to_moria.horizontal_backwards());
	all_traversals.extend(path_to_moria.vertical());
	all_traversals.extend(path_to_moria.vertical_backwards());
	all_traversals.extend(path_to_moria.diagonal());
	all_traversals.extend(path_to_moria.diagonal_backwards());
	all_traversals.retain(|arr| arr.len() >= 4);

	for traversal in all_traversals.iter() {
		let joined: String = traversal.iter().collect();
		if joined.contains("XMAS") {
			xmas_count += 1;
		}
	}

	println!("1. Instances of XMAS: {:?} (incorrect)", xmas_count);
}
