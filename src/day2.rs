
use nom::{
	IResult,
	bytes::complete::{tag, take},
	combinator::map_res,
	sequence::tuple,
};

pub fn solution_p1<S: AsRef<str>>(input: S) -> u32 {
	let mut total_score = 0;
	let rounds = input.as_ref().lines().map(|line| {
		parse_round_p1(line).and_then(|tup| Ok(tup.1))
	});
	for round in rounds {
		let your_score = round.unwrap().score();
		total_score += your_score;
	}
	total_score
}


pub fn solution_p2<S: AsRef<str>>(input: S) -> u32 {
	let mut total_score = 0;
	let rounds = input.as_ref().lines().map(|line| {
		parse_round_p2(line).and_then(|tup| Ok(tup.1))
	});
	for round in rounds {
		let your_score = round.unwrap().score();
		total_score += your_score;
	}
	total_score
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
	Rock,
	Paper,
	Scissors,
}

impl Shape {
    fn outcome(&self, other: &Self) -> Outcome {
    	match (self, other) {
			// Win conditions
			(Self::Rock, Self::Scissors) |
			(Self::Scissors, Self::Paper) | 
			(Self::Paper, Self::Rock) => Outcome::Win,
			// Lose conditions
			(Self::Scissors, Self::Rock) |
			(Self::Paper, Self::Scissors) |
			(Self::Rock, Self::Paper) => Outcome::Lose,
			// Rest of conditions are the same shapes
			_ => Outcome::Draw,
		}
	}

	fn value(&self) -> u32 {
		match self {
		    Shape::Rock => 1,
		    Shape::Paper => 2,
		    Shape::Scissors => 3,
		}
	}

	fn compliment(&self, desired_outcome: Outcome) -> Self {
		match (self, desired_outcome) {
			// Lose conditions
			(Self::Rock, Outcome::Lose) => Self::Scissors,
			(Self::Paper, Outcome::Lose) => Self::Rock,
			(Self::Scissors, Outcome::Lose) => Self::Paper,
			// Win conditions
			(Self::Rock, Outcome::Win) => Self::Paper,
			(Self::Paper, Outcome::Win) => Self::Scissors,
			(Self::Scissors, Outcome::Win) => Self::Rock,

			(_, Outcome::Draw) => *self,
		}
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Outcome {
	Lose,
	Draw,
	Win,
}

impl Outcome {
	fn value(&self) -> u32 {
		match self {
		    Outcome::Lose => 0,
		    Outcome::Draw => 3,
		    Outcome::Win => 6,
		}
	}
}

#[derive(Debug, PartialEq)]
struct Round {
	your_shape: Shape,
	their_shape: Shape,
	outcome: Outcome,
}

impl Round {
	fn from_part1_input(your_shape: Shape, their_shape: Shape) -> Self {
		Self {
			your_shape,
			their_shape,
			outcome: your_shape.outcome(&their_shape)
		}
	}
	fn from_part2_input(their_shape: Shape, outcome: Outcome) -> Self {
		Self {
			your_shape: their_shape.compliment(outcome),
			their_shape,
			outcome,
		}
	}
	fn score(&self) -> u32 {
		self.outcome.value() + self.your_shape.value()
	}
}

fn shape_from_char(shape: &str) -> Result<Shape, ()> {
	match shape {
		"A" | "X" => Ok(Shape::Rock),
		"B" | "Y" => Ok(Shape::Paper),
		"C" | "Z" => Ok(Shape::Scissors),
		_ => panic!(),
	}
}

fn outcome_from_char(outcome: &str) -> Result<Outcome, ()> {
	match outcome {
		"X" => Ok(Outcome::Lose),
		"Y" => Ok(Outcome::Draw),
		"Z" => Ok(Outcome::Win),
		_ => Err(())
	}
}

fn parse_shape(input: &str) -> IResult<&str, Shape> {
	map_res(take(1u8), shape_from_char)(input)
}

fn parse_outcome(input: &str) -> IResult<&str, Outcome> {
	map_res(take(1u8), outcome_from_char)(input)
}

fn parse_round_p1(input: &str) -> IResult<&str, Round> {
	let (input, (their_shape, _space, your_shape)) = tuple((parse_shape, tag(" "), parse_shape))(input)?;
	Ok((input, Round::from_part1_input(your_shape, their_shape)))
}

fn parse_round_p2(input: &str) -> IResult<&str, Round> {
	let (input, (their_shape, _space, outcome)) = tuple((parse_shape, tag(" "), parse_outcome))(input)?;
	Ok((input, Round::from_part2_input(their_shape, outcome)))
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::day2::{Round, Shape, parse_shape, solution_p1, Outcome};

    use super::parse_round_p1;

	#[test]
	fn example() {
		let input = "A Y
B X
C Z";
		assert_eq!(solution_p1(input), 15);
		let input = include_str!("../data/d2-example.txt");
		assert_eq!(solution_p1(input), 15);
	}

	#[test]
	fn test_parse_round() -> Result<(), Box<dyn Error>>{
		let input = "A X";
		let (input, round) = parse_round_p1(input)?;
		assert_eq!(input, "");
		assert_eq!(round, Round {
			their_shape: Shape::Rock,
			your_shape: Shape::Rock,
			outcome: Outcome::Draw,
		});
		let input = "B Z";
		let (input, round) = parse_round_p1(input)?;
		assert_eq!(input, "");
		assert_eq!(round, Round {
			their_shape: Shape::Paper,
			your_shape: Shape::Scissors,
			outcome: Outcome::Win
		});
		let input = "C Y";
		let (input, round) = parse_round_p1(input)?;
		assert_eq!(input, "");
		assert_eq!(round, Round {
			their_shape: Shape::Scissors,
			your_shape: Shape::Paper,
			outcome: Outcome::Lose,
		});
		Ok(())
	}

	#[test]
	fn test_parse_shape() -> Result<(), Box<dyn Error>> {
		let tests = [
			("A", Shape::Rock),
			("B", Shape::Paper),
			("C", Shape::Scissors),
			("X", Shape::Rock),
			("Y", Shape::Paper),
			("Z", Shape::Scissors),
		];
		for test in tests {
			let input = test.0;
			let expected = test.1;
			let (input, shape) = parse_shape(input)?;
			assert_eq!(input, "");
			assert_eq!(shape, expected);
		}
		Ok(())
	}
}
