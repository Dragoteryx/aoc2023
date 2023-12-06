use nom::IResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
	Red,
	Green,
	Blue,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Game {
	id: usize,
	sets: Vec<Vec<(usize, Color)>>,
}

impl Game {
	pub fn id(&self) -> usize {
		self.id
	}

	pub fn cubes(&self) -> impl Iterator<Item = (usize, Color)> + '_ {
		self.sets.iter().flat_map(|set| set.iter().copied())
	}

	pub fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
		self.cubes().all(|(count, color)| match color {
			Color::Red => red >= count,
			Color::Green => green >= count,
			Color::Blue => blue >= count,
		})
	}

	pub fn minimum_color_cubes(&self, color: Color) -> usize {
		self.cubes()
			.filter(|&(_, c)| c == color)
			.map(|(count, _)| count)
			.max()
			.unwrap_or(0)
	}

	pub fn minimum_power(&self) -> usize {
		let red = self.minimum_color_cubes(Color::Red);
		let green = self.minimum_color_cubes(Color::Green);
		let blue = self.minimum_color_cubes(Color::Blue);

		red * green * blue
	}

	pub fn parse_many(input: &str) -> impl Iterator<Item = IResult<&str, Self>> {
		input.lines().map(Self::parse)
	}

	pub fn parse(input: &str) -> IResult<&str, Self> {
		fn parse_color(input: &str) -> IResult<&str, Color> {
			use nom::branch::alt;
			use nom::bytes::complete::tag;
			use nom::combinator::map;

			alt((
				map(tag("red"), |_| Color::Red),
				map(tag("green"), |_| Color::Green),
				map(tag("blue"), |_| Color::Blue),
			))(input)
		}

		fn parse_color_quantity(input: &str) -> IResult<&str, (usize, Color)> {
			use nom::bytes::complete::tag;
			use nom::character::complete::digit1;
			use nom::combinator::map;
			use nom::sequence::separated_pair;

			map(
				separated_pair(digit1, tag(" "), parse_color),
				|(count, color)| (count.parse().unwrap(), color),
			)(input)
		}

		fn parse_color_quantities(input: &str) -> IResult<&str, Vec<(usize, Color)>> {
			use nom::bytes::complete::tag;
			use nom::multi::separated_list1;

			separated_list1(tag(", "), parse_color_quantity)(input)
		}

		fn parse_sets(input: &str) -> IResult<&str, Vec<Vec<(usize, Color)>>> {
			use nom::bytes::complete::tag;
			use nom::multi::separated_list1;

			separated_list1(tag("; "), parse_color_quantities)(input)
		}

		fn parse_game_id(input: &str) -> IResult<&str, usize> {
			use nom::bytes::complete::tag;
			use nom::character::complete::digit1;
			use nom::combinator::map;
			use nom::sequence::preceded;

			map(preceded(tag("Game "), digit1), |id: &str| {
				id.parse().unwrap()
			})(input)
		}

		fn parse_game_header(input: &str) -> IResult<&str, usize> {
			use nom::bytes::complete::tag;
			use nom::sequence::terminated;

			terminated(parse_game_id, tag(": "))(input)
		}

		let (input, id) = parse_game_header(input)?;
		let (input, sets) = parse_sets(input)?;

		Ok((input, Self { id, sets }))
	}
}
