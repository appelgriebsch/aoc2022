use std::{env, fs::File, io::Read, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item(char);

impl From<Item> for usize {
    fn from(item: Item) -> Self {
        match item.0 {
            'a'..='z' => (item.0 as usize) - 96,
            'A'..='Z' => (item.0 as usize) - 38,
            _ => 0,
        }
    }
}

struct Bag {
    left_compartment: Vec<Item>,
    right_compartment: Vec<Item>,
}

impl Bag {
    pub fn new(left: Vec<Item>, right: Vec<Item>) -> Self {
        Bag {
            left_compartment: left,
            right_compartment: right,
        }
    }
    pub fn find_duplicates(&self) -> Vec<Item> {
        let mut result = self.left_compartment.clone();
        result.retain(|&item| self.right_compartment.contains(&item));
        result.dedup();
        result
    }
    pub fn find_all_duplicates(bags: Vec<&Bag>) -> Vec<Item> {
        let mut result: Vec<Item> = bags[0].into();
        for bag in bags.iter().skip(1) {
            result.retain(|&value| Into::<Vec<Item>>::into(*bag).contains(&value));
        }
        result.dedup();
        result
    }
}

impl From<&Bag> for Vec<Item> {
    fn from(bag: &Bag) -> Self {
        let mut result = bag.left_compartment.clone();
        result.extend(&bag.right_compartment);
        result
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
        let bags: Vec<Bag> = input
            .split('\n')
            .map(|line| line.trim())
            .map(|line| {
                let breakpoint = line.len() / 2;
                let left: &str = &line[0..breakpoint];
                let right: &str = &line[breakpoint..];
                Bag::new(parse(left), parse(right))
            })
            .collect();
        let result: usize = bags
            .iter()
            .flat_map(Bag::find_duplicates)
            .map(Into::<usize>::into)
            .sum();
        println!("Final result is {result}");

        let mut groups: usize = 0;
        for chunk in bags.chunks(3) {
            let badges = Bag::find_all_duplicates(chunk.iter().collect());
            let group: usize = badges.into_iter().map(Into::<usize>::into).sum();
            groups += group;
        }
        println!("Final group result is {groups}");
    }
}

fn parse(line: &str) -> Vec<Item> {
    line.chars().map(Item).collect()
}

#[cfg(test)]
mod test {
    use crate::{parse, Bag, Item};

    #[test]
    fn value_lower_a() {
        let item = Item('a');
        let value: usize = item.into();
        assert_eq!(value, 1);
    }

    #[test]
    fn value_lower_z() {
        let item = Item('z');
        let value: usize = item.into();
        assert_eq!(value, 26);
    }

    #[test]
    fn value_upper_a() {
        let item = Item('A');
        let value: usize = item.into();
        assert_eq!(value, 27);
    }

    #[test]
    fn value_upper_z() {
        let item = Item('Z');
        let value: usize = item.into();
        assert_eq!(value, 52);
    }

    #[test]
    fn parse_sample() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
                        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                        PmmdzqPrVvPwwTWBwg
                        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                        ttgJtRGJQctTZtZT
                        CrZsJsPPZsGzwwsLwLmpwMDw"#;

        let bags: Vec<Bag> = input
            .split('\n')
            .map(|line| line.trim())
            .map(|line| {
                let breakpoint = line.len() / 2;
                let left: &str = &line[0..breakpoint];
                let right: &str = &line[breakpoint..];
                Bag::new(parse(left), parse(right))
            })
            .collect();

        let result: usize = bags
            .iter()
            .flat_map(Bag::find_duplicates)
            .map(Into::<usize>::into)
            .sum();
        assert_eq!(result, 157);
    }

    #[test]
    fn parse_sample_by_three() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
                        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                        PmmdzqPrVvPwwTWBwg
                        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                        ttgJtRGJQctTZtZT
                        CrZsJsPPZsGzwwsLwLmpwMDw"#;

        let bags: Vec<Bag> = input
            .split('\n')
            .map(|line| line.trim())
            .map(|line| {
                let breakpoint = line.len() / 2;
                let left: &str = &line[0..breakpoint];
                let right: &str = &line[breakpoint..];
                Bag::new(parse(left), parse(right))
            })
            .collect();

        let mut result: usize = 0;
        for chunk in bags.chunks(3) {
            let badges = Bag::find_all_duplicates(chunk.iter().collect());
            let group_result: usize = badges.into_iter().map(Into::<usize>::into).sum();
            result += group_result;
        }

        assert_eq!(result, 70);
    }
}
