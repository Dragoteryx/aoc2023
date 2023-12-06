mod parse;
use parse::*;

const INPUT: &str = include_str!("../inputs/real_input.txt");

fn part1(input: &str) -> usize {
	Game::parse_many(input)
		.filter_map(|res| res.ok().map(|res| res.1))
		.filter(|game| game.is_possible(12, 13, 14))
		.map(|game| game.id())
		.sum()
}

fn part2(input: &str) -> usize {
	Game::parse_many(input)
		.filter_map(|res| res.ok().map(|res| res.1))
		.map(|game| game.minimum_power())
		.sum()
}

fn main() {
	println!("--- day2 ---");
	println!("part1 => {}", part1(INPUT));
	println!("part2 => {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
	use super::{part1, part2};

	const INPUT: &str = include_str!("../inputs/test_input.txt");

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 8);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT), 2286);
	}
}
