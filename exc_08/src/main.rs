use std::{env, fs::File, io::Read, path::Path};

const RADIX: u32 = 10;

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
    }
}

#[cfg(test)]
mod test {
    use grid::Grid;

    use crate::RADIX;

    #[test]
    fn processes_sample_part1() {
        let input = r#"30373
                       25512
                       65332
                       33549
                       35390"#;
        let lines: Vec<_> = input.split('\n').map(|line| line.trim()).collect();
        let line_length = lines[0].chars().count();
        let mut grid: Grid<u32> = Grid::new(0, line_length);
        lines.iter()
             .for_each(|line| {
                grid.push_row(line.chars().map(|c| c.to_digit(RADIX).unwrap()).collect());
            });
        dbg!(&grid);
        assert_eq!(grid.get(0, 0), Some(&3));
        assert_eq!(grid.get(grid.rows() - 1, grid.cols() - 1), Some(&0));
    }
}
