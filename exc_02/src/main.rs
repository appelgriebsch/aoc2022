use std::{env, fs::File, io::Read, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl From<&str> for Shape {
    fn from(input: &str) -> Self {
        match input {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissor,
            _ => panic!("Unable to decode {input} value."),
        }
    }
}

impl From<Shape> for u8 {
    fn from(val: Shape) -> Self {
        match val {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Round {
    mine: Shape,
    theirs: Shape,
}

impl Round {
    pub fn new(mine: Shape, theirs: Shape) -> Self {
        Self { mine, theirs }
    }
    pub fn calculate_outcome(&self) -> usize {
        let value_mine: u8 = self.mine.into();

        let outcome = match self.get_tuple() {
            (Shape::Rock, Shape::Scissor) => 6,
            (Shape::Scissor, Shape::Paper) => 6,
            (Shape::Paper, Shape::Rock) => 6,
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Scissor, Shape::Scissor) => 3,
            (Shape::Paper, Shape::Paper) => 3,
            _ => 0,
        };

        (value_mine as usize) + (outcome as usize)
    }
    pub fn calculate_alternative(&self) -> Round {
        match self.get_tuple() {
            (Shape::Paper, theirs) => Round::new(theirs, theirs),
            (Shape::Rock, Shape::Scissor) => Round::new(Shape::Paper, Shape::Scissor),
            (Shape::Rock, Shape::Paper) => Round::new(Shape::Rock, Shape::Paper),
            (Shape::Rock, Shape::Rock) => Round::new(Shape::Scissor, Shape::Rock),
            (Shape::Scissor, Shape::Scissor) => Round::new(Shape::Rock, Shape::Scissor),
            (Shape::Scissor, Shape::Paper) => Round::new(Shape::Scissor, Shape::Paper),
            (Shape::Scissor, Shape::Rock) => Round::new(Shape::Paper, Shape::Rock),
        }
    }

    fn get_tuple(&self) -> (Shape, Shape) {
        (self.mine, self.theirs)
    }
}

impl From<&str> for Round {
    fn from(input_line: &str) -> Self {
        let values: Vec<Shape> = input_line.split(' ').map(Shape::from).collect();
        if values.len() != 2 {
            panic!("Unable to decode {input_line} into Shapes.");
        }

        Round::new(values[1], values[0])
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("Please provide input file.");
    }

    let input_filename = args[1].clone();

    if !Path::new(&input_filename).exists() {
        panic!("Input file {input_filename} does not exists or is not accessible.");
    }

    let mut input = String::new();
    if let Ok(mut input_file) = File::open(&args[1]) {
        input_file
            .read_to_string(&mut input)
            .expect("Error reading input file {input_filename}.");
        let rounds: Vec<Round> = input
            .split('\n')
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(Round::from)
            .collect();

        let result: usize = rounds.iter().map(|round| round.calculate_outcome()).sum();
        println!("Final result of your chosen strategy: {result}");

        let alternative_result: usize = rounds
            .iter()
            .map(|round| round.calculate_alternative())
            .map(|round| round.calculate_outcome())
            .sum();
        println!("Alternative result of your chosen strategy: {alternative_result}");
    }
}

#[cfg(test)]
mod test {
    use crate::{Round, Shape};

    #[test]
    fn parse_values() {
        let input = r#"A Y
                       B X
                       C Z"#;
        let rounds: Vec<Round> = input
            .split('\n')
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(Round::from)
            .collect();
        assert_eq!(
            rounds,
            vec![
                Round::new(Shape::Paper, Shape::Rock),
                Round::new(Shape::Rock, Shape::Paper),
                Round::new(Shape::Scissor, Shape::Scissor)
            ]
        )
    }

    #[test]
    fn calculates_outcome() {
        let rounds = vec![
            Round::new(Shape::Paper, Shape::Rock),
            Round::new(Shape::Rock, Shape::Paper),
            Round::new(Shape::Scissor, Shape::Scissor),
        ];
        let result: usize = rounds
            .into_iter()
            .map(|round| round.calculate_outcome())
            .sum();
        assert_eq!(result, 15);
    }

    #[test]
    fn calculates_alternative_1() {
        let round = Round::new(Shape::Paper, Shape::Rock);
        let alternative = round.calculate_alternative();
        assert_eq!(alternative, Round::new(Shape::Rock, Shape::Rock));
    }

    #[test]
    fn calculates_alternative_2() {
        let round = Round::new(Shape::Rock, Shape::Paper);
        let alternative = round.calculate_alternative();
        assert_eq!(alternative, Round::new(Shape::Rock, Shape::Paper));
    }

    #[test]
    fn calculates_alternative_3() {
        let round = Round::new(Shape::Scissor, Shape::Scissor);
        let alternative = round.calculate_alternative();
        assert_eq!(alternative, Round::new(Shape::Rock, Shape::Scissor));
    }

    #[test]
    fn calculates_alternative() {
        let rounds = vec![
            Round::new(Shape::Paper, Shape::Rock),
            Round::new(Shape::Rock, Shape::Paper),
            Round::new(Shape::Scissor, Shape::Scissor),
        ];
        let result: usize = rounds
            .into_iter()
            .map(|round| round.calculate_alternative())
            .map(|round| round.calculate_outcome())
            .sum();
        assert_eq!(result, 12);
    }
}
