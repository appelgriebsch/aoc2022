use regex::Regex;
use std::{env, fmt::Display, fs::File, io::Read, path::Path};

#[derive(Debug, Clone)]
struct Slot {
    items: Vec<char>,
}

impl Slot {
    pub fn new(items: Vec<char>) -> Self {
        Slot { items }
    }
    pub fn pop(&mut self) -> Option<char> {
        self.items.pop()
    }
    pub fn pop_n(&mut self, no_of_items: usize) -> Vec<char> {
        let mut result = Vec::with_capacity(no_of_items);
        for _ in 0..no_of_items {
            if let Some(c) = self.pop() {
                result.push(c);
            }
        }
        result
    }
    pub fn push(&mut self, item: char) {
        self.items.push(item)
    }
    pub fn push_n(&mut self, items: &[char]) {
        for item in items {
            self.push(item.to_owned());
        }
    }
    #[must_use]
    pub fn items(&self) -> &[char] {
        self.items.as_ref()
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.items().iter() {
            f.write_str(&format!("{} ", item))?;
        }
        f.write_str("")
    }
}

#[derive(Debug)]
struct Stock {
    slots: Vec<Slot>,
}

impl Stock {
    pub fn new(slots: Vec<Slot>) -> Self {
        Stock { slots }
    }

    pub fn shift(&mut self, from: usize, to: usize, no_of_items: usize) {
        let mut chars: Vec<char> = Vec::new();
        if let Some(from_slot) = self.slots.get_mut(from - 1) {
            chars = from_slot.pop_n(no_of_items);
        }
        if let Some(to_slot) = self.slots.get_mut(to - 1) {
            to_slot.push_n(&chars);
        }
    }

    pub fn bulk_move(&mut self, from: usize, to: usize, no_of_items: usize) {
        let mut chars: Vec<char> = Vec::new();
        if let Some(from_slot) = self.slots.get_mut(from - 1) {
            chars = from_slot.pop_n(no_of_items);
        }
        chars.reverse();
        if let Some(to_slot) = self.slots.get_mut(to - 1) {
            to_slot.push_n(&chars);
        }
    }

    #[must_use]
    pub fn slots(&self) -> &[Slot] {
        self.slots.as_ref()
    }
}

impl Display for Stock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for slot in self.slots().iter() {
            f.write_str(&format!("| {}|\n", slot))?;
        }
        f.write_str("")
    }
}

fn run_shifts(input_filename: &str) {
    let slot_1 = Slot::new(vec!['Q', 'M', 'G', 'C', 'L']);
    let slot_2 = Slot::new(vec!['R', 'D', 'L', 'C', 'T', 'F', 'H', 'G']);
    let slot_3 = Slot::new(vec!['V', 'J', 'F', 'N', 'M', 'T', 'W', 'R']);
    let slot_4 = Slot::new(vec!['J', 'F', 'D', 'V', 'Q', 'P']);
    let slot_5 = Slot::new(vec!['N', 'F', 'M', 'S', 'L', 'B', 'T']);
    let slot_6 = Slot::new(vec!['R', 'N', 'V', 'H', 'C', 'D', 'P']);
    let slot_7 = Slot::new(vec!['H', 'C', 'T']);
    let slot_8 = Slot::new(vec!['G', 'S', 'J', 'V', 'Z', 'N', 'H', 'P']);
    let slot_9 = Slot::new(vec!['Z', 'F', 'H', 'G']);
    let mut stock = Stock::new(vec![
        slot_1, slot_2, slot_3, slot_4, slot_5, slot_6, slot_7, slot_8, slot_9,
    ]);

    let mut input = String::new();
    if let Ok(mut input_file) = File::open(input_filename) {
        input_file
            .read_to_string(&mut input)
            .expect("Error reading input file {input_filename}.");

        let re = Regex::new(
            r"^move (?P<no_of_items>\d{1,2}) from (?P<from_slot>\d{1}) to (?P<to_slot>\d{1})$",
        )
        .expect("regex is wrong!");
        let mut counter = 0;
        input
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && re.is_match(line))
            .for_each(|instruction| {
                if let Some(captures) = re.captures(instruction) {
                    let no_of_items: usize = captures["no_of_items"].parse().unwrap();
                    let from_slot: usize = captures["from_slot"].parse().unwrap();
                    let to_slot: usize = captures["to_slot"].parse().unwrap();
                    stock.shift(from_slot, to_slot, no_of_items);
                    counter += 1;
                }
            });
        println!("After {counter} shifts stock looks like this: ");
        println!("{}", stock);
    }
}

fn run_bulk_moves(input_filename: &str) {
    let slot_1 = Slot::new(vec!['Q', 'M', 'G', 'C', 'L']);
    let slot_2 = Slot::new(vec!['R', 'D', 'L', 'C', 'T', 'F', 'H', 'G']);
    let slot_3 = Slot::new(vec!['V', 'J', 'F', 'N', 'M', 'T', 'W', 'R']);
    let slot_4 = Slot::new(vec!['J', 'F', 'D', 'V', 'Q', 'P']);
    let slot_5 = Slot::new(vec!['N', 'F', 'M', 'S', 'L', 'B', 'T']);
    let slot_6 = Slot::new(vec!['R', 'N', 'V', 'H', 'C', 'D', 'P']);
    let slot_7 = Slot::new(vec!['H', 'C', 'T']);
    let slot_8 = Slot::new(vec!['G', 'S', 'J', 'V', 'Z', 'N', 'H', 'P']);
    let slot_9 = Slot::new(vec!['Z', 'F', 'H', 'G']);
    let mut stock = Stock::new(vec![
        slot_1, slot_2, slot_3, slot_4, slot_5, slot_6, slot_7, slot_8, slot_9,
    ]);

    let mut input = String::new();
    if let Ok(mut input_file) = File::open(input_filename) {
        input_file
            .read_to_string(&mut input)
            .expect("Error reading input file {input_filename}.");

        let re = Regex::new(
            r"^move (?P<no_of_items>\d{1,2}) from (?P<from_slot>\d{1}) to (?P<to_slot>\d{1})$",
        )
        .expect("regex is wrong!");
        let mut counter = 0;
        input
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && re.is_match(line))
            .for_each(|instruction| {
                if let Some(captures) = re.captures(instruction) {
                    let no_of_items: usize = captures["no_of_items"].parse().unwrap();
                    let from_slot: usize = captures["from_slot"].parse().unwrap();
                    let to_slot: usize = captures["to_slot"].parse().unwrap();
                    stock.bulk_move(from_slot, to_slot, no_of_items);
                    counter += 1;
                }
            });
        println!("After {counter} bulk_moves stock looks like this: ");
        println!("{}", stock);
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

    run_shifts(&input_filename);
    run_bulk_moves(&input_filename);
}

#[cfg(test)]
mod test {
    use crate::{Slot, Stock};

    #[test]
    fn processes_sample_part1() {
        /*
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        */
        let slot_1 = Slot::new(vec!['Z', 'N']);
        let slot_2 = Slot::new(vec!['M', 'C', 'D']);
        let slot_3 = Slot::new(vec!['P']);

        let mut stock = Stock::new(vec![slot_1, slot_2, slot_3]);
        assert_eq!(stock.to_string(), "| Z N |\n| M C D |\n| P |\n");

        // step 1: move 1 from 2 to 1
        stock.shift(2, 1, 1);
        assert_eq!(stock.to_string(), "| Z N D |\n| M C |\n| P |\n");

        // step 2: move 3 from 1 to 3
        stock.shift(1, 3, 3);
        assert_eq!(stock.to_string(), "| |\n| M C |\n| P D N Z |\n");

        // step 3: move 2 from 2 to 1
        stock.shift(2, 1, 2);
        assert_eq!(stock.to_string(), "| C M |\n| |\n| P D N Z |\n");

        // step 4: move 1 from 1 to 2
        stock.shift(1, 2, 1);
        assert_eq!(stock.to_string(), "| C |\n| M |\n| P D N Z |\n");
    }

    #[test]
    fn processes_sample_part2() {
        /*
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        */
        let slot_1 = Slot::new(vec!['Z', 'N']);
        let slot_2 = Slot::new(vec!['M', 'C', 'D']);
        let slot_3 = Slot::new(vec!['P']);

        let mut stock = Stock::new(vec![slot_1, slot_2, slot_3]);
        assert_eq!(stock.to_string(), "| Z N |\n| M C D |\n| P |\n");

        // step 1: move 1 from 2 to 1
        stock.bulk_move(2, 1, 1);
        assert_eq!(stock.to_string(), "| Z N D |\n| M C |\n| P |\n");

        // step 2: move 3 from 1 to 3
        stock.bulk_move(1, 3, 3);
        assert_eq!(stock.to_string(), "| |\n| M C |\n| P Z N D |\n");

        // step 3: move 2 from 2 to 1
        stock.bulk_move(2, 1, 2);
        assert_eq!(stock.to_string(), "| M C |\n| |\n| P Z N D |\n");

        // step 4: move 1 from 1 to 2
        stock.bulk_move(1, 2, 1);
        assert_eq!(stock.to_string(), "| M |\n| C |\n| P Z N D |\n");
    }
}
