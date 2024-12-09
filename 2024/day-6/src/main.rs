use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Debug)]
struct Grid {
    data: Vec<char>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

impl Grid {
	fn from_file(path: &str) -> io::Result<Self> {
		let file = File::open(path)?;
		let lines: Vec<String> = io::BufReader::new(file)
			.lines()
			.collect::<Result<_, _>>()?;

		if lines.is_empty() {
			return Err(io::Error::new(io::ErrorKind::InvalidData, "Empty file"));
		}

		let height = lines.len();
		let width = lines[0].len();
		let data: Vec<char> = lines.iter()
			.flat_map(|line| line.chars())
			.collect();

		Ok(Grid { data, width, height })
	}

	fn get(&self, x: usize, y: usize) -> Option<char> {
		if x < self.width && y < self.height {
			Some(self.data[y * self.width + x])
		} else {
			None
		}
	}

	fn find_start(&self) -> Option<(usize, usize)> {
		for y in 0..self.height {
			for x in 0..self.width {
				if self.get(x, y) == Some('^') {
					return Some((x, y));
				}
			}
		}
		None
	}

	fn is_inside(&self, x: i32, y: i32) -> bool {
		x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
	}

	fn simulate_guard_path(&self) -> usize {
		let (start_x, start_y) = self.find_start().unwrap();
		let mut visited = HashSet::new();
		let mut current_pos = (start_x as i32, start_y as i32);
		let mut direction = Direction::Up;

		visited.insert(current_pos);

		loop {
			let (dx, dy) = direction.get_delta();
			let next_pos = (current_pos.0 + dx, current_pos.1 + dy);

			if self.is_inside(next_pos.0, next_pos.1) {
				let next_char = self.get(next_pos.0 as usize, next_pos.1 as usize).unwrap();
				if next_char == '#' {
					direction = direction.turn_right();
				} else {
					current_pos = next_pos;
					visited.insert(current_pos);
				}
			} else {
				break
			}
		}
		visited.len()
	}
}

fn main() -> io::Result<()> {
	let grid = Grid::from_file("src/input.txt")?;
	let result = grid.simulate_guard_path();

	println!("1. Distinct positions the guard visits: {}", result);
	Ok(())
}
