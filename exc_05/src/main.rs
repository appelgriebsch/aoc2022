use std::{
    env,
    fmt::{Display, Write},
    fs::File,
    io::Read,
    path::Path,
};

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
    #[must_use]
    pub fn slots(&self) -> &[Slot] {
        self.slots.as_ref()
    }
}

impl Display for Stock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for slot in self.slots().iter() {
            f.write_str(&format!("| {}", slot))?;
        }
        f.write_char('|')
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
    }
}

#[cfg(test)]
mod test {
    use crate::{Slot, Stock};

    #[test]
    fn processes_sample() {
        let slot_1 = Slot::new(vec!['Z', 'N']);
        let slot_2 = Slot::new(vec!['M', 'C', 'D']);
        let slot_3 = Slot::new(vec!['P']);

        let mut stock = Stock::new(vec![slot_1, slot_2, slot_3]);
        assert_eq!(stock.to_string(), "| Z N | M C D | P |");

        // step 1: move 1 from 2 to 1
        stock.shift(2, 1, 1);
        assert_eq!(stock.to_string(), "| Z N D | M C | P |");

        // step 2: move 3 from 1 to 3
        stock.shift(1, 3, 3);
        assert_eq!(stock.to_string(), "| | M C | P D N Z |");

        // step 3: move 2 from 2 to 1
        stock.shift(2, 1, 2);
        assert_eq!(stock.to_string(), "| C M | | P D N Z |");

        // step 4: move 1 from 1 to 2
        stock.shift(1, 2, 1);
        assert_eq!(stock.to_string(), "| C | M | P D N Z |");
    }
}
