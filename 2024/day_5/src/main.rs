use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_rules<P>(filename: P) -> io::Result<Vec<(u32, u32)>>
where
	P: AsRef<Path>,
{
	let file = File::open(filename)?;
	let reader = io::BufReader::new(file);
	let mut rules = Vec::new();

	for line in reader.lines() {
		let line = line?;
		let parts: Vec<&str> = line.split('|').collect();
		if parts.len() == 2 {
			if let (Ok(from), Ok(to)) = (parts[0].parse(), parts[1].parse()) {
				rules.push((from, to));
			}
		}
	}

	Ok(rules)
}

fn parse_numbers<P>(filename: P) -> io::Result<Vec<Vec<u32>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut updates = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<u32> = line
            .split(',')
            .filter_map(|n| n.trim().parse().ok())
            .collect();
        if !numbers.is_empty() {
            updates.push(numbers);
        }
    }

    Ok(updates)
}

fn adjacency_list(update: &[u32], relevant_rules: &[(u32, u32)]) -> (HashMap<u32, Vec<u32>>, HashMap<u32, usize>) {
	let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
	let mut in_degree: HashMap<u32, usize> = HashMap::new();

	for &num in update {
		graph.entry(num).or_default();
		in_degree.entry(num).or_insert(0);
	}

	for &(from, to) in relevant_rules {
		graph.entry(from).or_default().push(to);
		*in_degree.entry(to).or_insert(0) += 1;
	}

	(graph, in_degree)
}

fn topological_sort(update: &[u32], graph: &HashMap<u32, Vec<u32>>, in_degree: &mut HashMap<u32, usize>) -> Vec<u32> {
	let mut queue: VecDeque<_> = update
		.iter()
		.copied()
		.filter(|&num| in_degree[&num] == 0)
		.collect();

	let mut sorted = Vec::new();

	while let Some(node) = queue.pop_front() {
		sorted.push(node);

		if let Some(neighbors) = graph.get(&node) {
			for &next in neighbors {
				*in_degree.get_mut(&next).unwrap() -= 1;
				if in_degree[&next] == 0 {
					queue.push_back(next);
				}
			}
		}
	}

	sorted
}

fn is_valid_order(rules: &[(u32, u32)], update: &[u32]) -> bool {
	let update_set: HashSet<_> = update.iter().copied().collect();

	let relevant_rules: Vec<_> = rules
		.iter()
		.filter(|&&(from, to)| update_set.contains(&from) && update_set.contains(&to))
		.copied()
		.collect();

	let (graph, mut in_degree) = adjacency_list(update, &relevant_rules);
	let sorted = topological_sort(update, &graph, &mut in_degree);

	sorted == update
}

fn main() -> io::Result<()> {
	let rules = parse_rules("src/page_ordering_rules.txt")?;
	let updates = parse_numbers("src/page_numbers.txt")?;

	let mut sum = 0;
	for update in &updates {
		if is_valid_order(&rules, update) {
			let middle_index = update.len() / 2;
			sum += update[middle_index];
		}
	}

	println!("1. Sum of valid middle page numbers: {}", sum);
	Ok(())
}
