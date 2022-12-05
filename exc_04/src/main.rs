use std::{env, fs::File, io::Read, path::Path};

#[derive(Debug, Clone, Copy)]
struct SectionAssignment {
    lower_end: u8,
    upper_end: u8,
}

impl SectionAssignment {
    pub fn new(lower_end: u8, upper_end: u8) -> Self {
        SectionAssignment {
            lower_end,
            upper_end,
        }
    }
}

impl From<&str> for SectionAssignment {
    fn from(input: &str) -> Self {
        let (low, up) = input.split_once('-').expect(input);
        SectionAssignment::new(
            str::parse::<u8>(low).expect(low),
            str::parse::<u8>(up).expect(up),
        )
    }
}

struct Pair {
    left: SectionAssignment,
    right: SectionAssignment,
}

impl Pair {
    pub fn new(left: SectionAssignment, right: SectionAssignment) -> Self {
        Pair { left, right }
    }
    pub fn is_contained(&self) -> bool {
        (self.left.lower_end <= self.right.lower_end && self.left.upper_end >= self.right.upper_end)
            || (self.right.lower_end <= self.left.lower_end
                && self.right.upper_end >= self.left.upper_end)
    }
    pub fn is_overlapping(&self) -> bool {
        (self.left.lower_end <= self.right.lower_end && self.left.upper_end >= self.right.lower_end)
            || (self.right.lower_end <= self.left.lower_end
                && self.right.upper_end >= self.left.lower_end)
    }
}

impl From<&str> for Pair {
    fn from(input: &str) -> Self {
        let assignments: Vec<_> = input
            .split(',')
            .map(Into::<SectionAssignment>::into)
            .collect();
        Pair::new(assignments[0], assignments[1])
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
    if let Ok(mut input_file) = File::open(&input_filename) {
        input_file
            .read_to_string(&mut input)
            .expect("Error reading input file {input_filename}.");

        let pairs: Vec<Pair> = input
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.into())
            .collect();

        let count = pairs.iter().filter(|pair| pair.is_contained()).count();
        println!("Count of contained sections: {count}");

        let count2 = pairs.iter().filter(|pair| pair.is_overlapping()).count();
        println!("Count of overlapping sections: {count2}");
    }
}

#[cfg(test)]
mod test {
    use crate::Pair;

    #[test]
    fn processes_sample() {
        let input = r#"2-4,6-8
                       2-3,4-5
                       5-7,7-9
                       2-8,3-7
                       6-6,4-6
                       2-6,4-8"#;
        let pairs: Vec<Pair> = input
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.into())
            .collect();
        let count = pairs.iter().filter(|pair| pair.is_contained()).count();
        assert_eq!(count, 2);

        let count2 = pairs.iter().filter(|pair| pair.is_overlapping()).count();
        assert_eq!(count2, 4);
    }
}
