use std::{fs::File, env, path::Path, io::Read};

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
        input_file.read_to_string(& mut input).expect("Error reading input file {input_filename}.");
        let mut elves: Vec<_> = input.split("\n\n")
                                     .map(calculate_calories)
                                     .collect();
        elves.sort();
        elves.reverse();
        println!("Elf with largest calories: {}", elves[0]);
        let top3_elves: usize = elves.into_iter()
                                     .take(3)
                                     .sum();
        println!("Top 3 elves carrying {} calories", top3_elves);
    }
}

fn calculate_calories(elf: &str) -> usize {
    elf.split('\n')
       .filter(|value| !value.is_empty())
       .map(|value| value.parse::<usize>().unwrap())
       .sum()
}
