use std::{env, fs::File, io::Read, path::Path};

struct Slot {
    items: Vec<char>,
}

impl Slot {
    pub fn new(items: &[char]) -> Self {
        Slot {
            items: items.to_vec(),
        }
    }
    pub fn pop(&mut self) -> Option<char> {
        self.items.pop()
    }
    pub fn push(&mut self, item: char) {
        self.items.push(item)
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
mod test {}
