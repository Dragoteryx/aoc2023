const INPUT: &str = include_str!("../inputs/real_input.txt");

fn part1(input: &str) -> u32 {
	input
		.lines()
		.map(|line| line.chars().filter(|c| c.is_ascii_digit()))
		.map(|mut line| {
			let first = line.next().unwrap();
			let last = line.last().unwrap_or(first);
			format!("{first}{last}").parse::<u32>().unwrap()
		})
		.sum()
}

fn part2(input: &str) -> u32 {
	fn digit_to_str(digit: u8) -> &'static str {
		match digit {
			0 => "zero",
			1 => "one",
			2 => "two",
			3 => "three",
			4 => "four",
			5 => "five",
			6 => "six",
			7 => "seven",
			8 => "eight",
			9 => "nine",
			_ => panic!("invalid digit"),
		}
	}

	fn str_to_digit(s: &str) -> u32 {
		match s {
			"0" | "zero" => 0,
			"1" | "one" => 1,
			"2" | "two" => 2,
			"3" | "three" => 3,
			"4" | "four" => 4,
			"5" | "five" => 5,
			"6" | "six" => 6,
			"7" | "seven" => 7,
			"8" | "eight" => 8,
			"9" | "nine" => 9,
			_ => panic!("invalid digit"),
		}
	}

	fn patterns() -> impl Iterator<Item = String> {
		(0..=9)
			.map(|i| i.to_string())
			.chain((0..=9).map(digit_to_str).map(|i| i.to_string()))
	}

	input
		.lines()
		.map(|line| {
			let mut matches = patterns()
				.flat_map(|pattern| {
					let mut matches = line.match_indices(&pattern);
					let first = matches.next();
					let last = matches.last();
					[first, last].into_iter().flatten()
				})
				.collect::<Vec<_>>();
			matches.sort_by_key(|(i, _)| *i);
			let first = str_to_digit(matches.first().unwrap().1);
			let last = str_to_digit(matches.last().unwrap().1);
			format!("{}{}", first, last).parse::<u32>().unwrap()
		})
		.sum()
}

fn main() {
	println!("--- day 1 ---");
	println!("part 1 => {}", part1(INPUT));
	println!("part 2 => {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
	use super::{part1, part2};

	const INPUT: &str = include_str!("../inputs/test_input.txt");
	const INPUT2: &str = include_str!("../inputs/test_input2.txt");

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 142);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT2), 281);
	}
}
