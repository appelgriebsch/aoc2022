use std::{collections::HashSet, env, fs::File, io::Read, path::Path};

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn update(&mut self, direction: Direction) -> Vec<Position> {
        let mut visited_positions = Vec::new();
        match direction {
            Direction::Up(u) => {
                for i in 0..u {
                    visited_positions.push(Position::new(self.x, self.y + i));
                }
                self.y += u
            }
            Direction::Down(d) => {
                for i in 0..d {
                    visited_positions.push(Position::new(self.x, self.y - i));
                }
                self.y -= d
            }
            Direction::Left(l) => {
                for i in 0..l {
                    visited_positions.push(Position::new(self.x - i, self.y));
                }
                self.x -= l
            }
            Direction::Right(r) => {
                for i in 0..r {
                    visited_positions.push(Position::new(self.x + i, self.y));
                }
                self.x += r
            }
            Direction::Other(x, y) => {
                for i in 0..x {
                    visited_positions.push(Position::new(self.x + i, self.y));
                }
                for i in 0..y {
                    visited_positions.push(Position::new(self.x, self.y + i));
                }
                self.x += x;
                self.y += y;
            }
        }
        visited_positions
    }

    pub fn difference(&self, other: &Position) -> Direction {
        if self.x == other.x && self.y > other.y {
            Direction::Up(self.y - other.y)
        } else if self.x == other.x && self.y < other.y {
            Direction::Down(self.y - other.y)
        } else if self.y == other.y && self.x > other.x {
            Direction::Right(self.x - other.x)
        } else if self.y == other.y && self.x < other.y {
            Direction::Left(self.x - other.x)
        } else {
            Direction::Other(self.x - other.x, self.y - other.y)
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
    Other(i32, i32),
}

impl TryFrom<&str> for Direction {
    type Error = MovementError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace().rev();
        let count: i32 = match parts.next() {
            Some(c) => c
                .parse()
                .map_err(|_| MovementError("Error decoding movement pattern: Invalid count."))?,
            None => 0,
        };
        match parts.next() {
            Some("R") => Ok(Direction::Right(count)),
            Some("L") => Ok(Direction::Left(count)),
            Some("U") => Ok(Direction::Up(count)),
            Some("D") => Ok(Direction::Down(count)),
            _ => Err(MovementError(
                "Error decoding movement pattern: Invalid direction.",
            )),
        }
    }
}

#[derive(Debug)]
pub struct MovementError(&'static str);

#[derive(Debug)]
pub struct Board {
    start: Position,
    head: Position,
    tail: Position,
    tail_positions: HashSet<Position>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        let mut positions = HashSet::default();
        positions.insert(Position::default());

        Board {
            start: Position::default(),
            head: Position::default(),
            tail: Position::default(),
            tail_positions: positions,
        }
    }

    pub fn start(&self) -> &Position {
        &self.start
    }

    pub fn head(&self) -> &Position {
        &self.head
    }

    pub fn tail(&self) -> &Position {
        &self.tail
    }

    pub fn tail_positions(&self) -> &HashSet<Position> {
        &self.tail_positions
    }

    pub fn update(&mut self, direction: Direction) -> Result<(), MovementError> {
        self.head.update(direction);
        let direction = self.head.difference(&self.tail);
        for pos in self.tail.update(direction) {
            self.tail_positions.insert(pos);
        }
        Ok(())
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
    if let Ok(mut input_file) = File::open(input_filename) {
        input_file
            .read_to_string(&mut input)
            .expect("Error reading input file {input_filename}.");

        let mut board = Board::new();

        let lines: Vec<_> = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();

        for line in lines {
            let direction: Direction = line.try_into().unwrap();
            dbg!(&direction);
            board.update(direction).unwrap();
        }

        println!(
            "Number of unique positions for tail: {}",
            board.tail_positions().len()
        );
    }
}

#[cfg(test)]
mod test {
    use crate::{Board, Direction};

    #[test]
    fn processes_sample1() {
        let input = r#"R 4
                       U 4
                       L 3
                       D 1
                       R 4
                       D 1
                       L 5
                       R 2"#;

        let mut board = Board::new();

        let lines: Vec<_> = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();

        for line in lines {
            let direction: Direction = line.try_into().unwrap();
            board.update(direction).unwrap();
        }

        dbg!(board.tail_positions());
        assert_eq!(board.tail_positions().len(), 13);
    }
}
