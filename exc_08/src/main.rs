use std::{env, fs::File, io::Read, path::Path};

use grid::Grid;

const RADIX: u32 = 10;

fn is_visible_from_top(grid: &Grid<u32>, row: usize, col: usize) -> bool {
    let cell = grid[row][col];
    grid.iter_col(col).take(row).all(|c| *c < cell)
}

fn is_visible_from_bottom(grid: &Grid<u32>, row: usize, col: usize) -> bool {
    let cell = grid[row][col];
    grid.iter_col(col).skip(row + 1).all(|c| *c < cell)
}

fn is_visible_from_left(grid: &Grid<u32>, row: usize, col: usize) -> bool {
    let cell = grid[row][col];
    grid.iter_row(row).take(col).all(|c| *c < cell)
}

fn is_visible_from_right(grid: &Grid<u32>, row: usize, col: usize) -> bool {
    let cell = grid[row][col];
    grid.iter_row(row).skip(col + 1).all(|c| *c < cell)
}

fn view_score_from_top(grid: &Grid<u32>, row: usize, col: usize) -> usize {
    let cell = grid[row][col];
    let cells: Vec<_> = grid.iter_col(col).take(row).rev().collect();
    let mut result = 0;
    for val in cells.into_iter() {
        result += 1;
        if *val >= cell {
            break;
        }
    }
    result
}

fn view_score_from_left(grid: &Grid<u32>, row: usize, col: usize) -> usize {
    let cell = grid[row][col];
    let cells: Vec<_> = grid.iter_row(row).take(col).rev().collect();
    let mut result = 0;
    for val in cells.into_iter() {
        result += 1;
        if *val >= cell {
            break;
        }
    }
    result
}

fn view_score_from_right(grid: &Grid<u32>, row: usize, col: usize) -> usize {
    let cell = grid[row][col];
    let cells: Vec<_> = grid.iter_row(row).skip(col + 1).collect();
    let mut result = 0;
    for val in cells.into_iter() {
        result += 1;
        if *val >= cell {
            break;
        }
    }
    result
}

fn view_score_from_bottom(grid: &Grid<u32>, row: usize, col: usize) -> usize {
    let cell = grid[row][col];
    let cells: Vec<_> = grid.iter_col(col).skip(row + 1).collect();
    let mut result = 0;
    for val in cells.into_iter() {
        result += 1;
        if *val >= cell {
            break;
        }
    }
    result
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

        let lines: Vec<_> = input
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();

        let line_length = lines[0].chars().count();
        let mut grid: Grid<u32> = Grid::new(0, line_length);
        lines.iter().for_each(|line| {
            grid.push_row(line.chars().map(|c| c.to_digit(RADIX).unwrap()).collect());
        });

        let outer = grid.iter_col(0).count()
            + grid.iter_col(grid.cols() - 1).count()
            + grid.iter_row(0).skip(1).take(grid.cols() - 2).count()
            + grid
                .iter_row(grid.rows() - 1)
                .skip(1)
                .take(grid.cols() - 2)
                .count();

        let mut inner = 0;
        for row in 1..(grid.rows() - 1) {
            for col in 1..(grid.cols() - 1) {
                if is_visible_from_top(&grid, row, col)
                    || is_visible_from_left(&grid, row, col)
                    || is_visible_from_right(&grid, row, col)
                    || is_visible_from_bottom(&grid, row, col)
                {
                    inner += 1;
                }
            }
        }

        println!("There are {outer} outer and {inner} inner trees visible");

        let mut view_score: Vec<_> = Vec::new();
        for row in 1..(grid.rows() - 1) {
            for col in 1..(grid.cols() - 1) {
                let score = view_score_from_top(&grid, row, col)
                    * view_score_from_left(&grid, row, col)
                    * view_score_from_right(&grid, row, col)
                    * view_score_from_bottom(&grid, row, col);
                view_score.push(score);
            }
        }

        println!(
            "The maximum view score is {}",
            view_score.into_iter().max().unwrap()
        );
    }
}

#[cfg(test)]
mod test {
    use grid::Grid;

    use crate::{
        is_visible_from_bottom, is_visible_from_left, is_visible_from_right, is_visible_from_top,
        view_score_from_bottom, view_score_from_left, view_score_from_right, view_score_from_top,
        RADIX,
    };

    #[test]
    fn processes_sample_part1() {
        let input = r#"30373
                       25512
                       65332
                       33549
                       35390"#;
        let lines: Vec<_> = input
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();
        let line_length = lines[0].chars().count();
        let mut grid: Grid<u32> = Grid::new(0, line_length);
        lines.iter().for_each(|line| {
            grid.push_row(line.chars().map(|c| c.to_digit(RADIX).unwrap()).collect());
        });
        dbg!(&grid);
        assert_eq!(grid.get(0, 0), Some(&3));
        assert_eq!(grid.get(grid.rows() - 1, grid.cols() - 1), Some(&0));
        let outer = grid.iter_col(0).count()
            + grid.iter_col(grid.cols() - 1).count()
            + grid.iter_row(0).skip(1).take(grid.cols() - 2).count()
            + grid
                .iter_row(grid.rows() - 1)
                .skip(1)
                .take(grid.cols() - 2)
                .count();
        assert_eq!(outer, 16);

        let mut inner = 0;
        for row in 1..(grid.rows() - 1) {
            for col in 1..(grid.cols() - 1) {
                if is_visible_from_top(&grid, row, col)
                    || is_visible_from_left(&grid, row, col)
                    || is_visible_from_right(&grid, row, col)
                    || is_visible_from_bottom(&grid, row, col)
                {
                    inner += 1;
                }
            }
        }

        assert_eq!(inner, 5);
    }

    #[test]
    fn processes_sample_part2() {
        let input = r#"30373
                       25512
                       65332
                       33549
                       35390"#;
        let lines: Vec<_> = input
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();
        let line_length = lines[0].chars().count();
        let mut grid: Grid<u32> = Grid::new(0, line_length);
        lines.iter().for_each(|line| {
            grid.push_row(line.chars().map(|c| c.to_digit(RADIX).unwrap()).collect());
        });
        dbg!(&grid);
        assert_eq!(grid.get(0, 0), Some(&3));
        assert_eq!(grid.get(grid.rows() - 1, grid.cols() - 1), Some(&0));

        let mut view_score: Vec<_> = Vec::new();
        for row in 1..(grid.rows() - 1) {
            for col in 1..(grid.cols() - 1) {
                let score = view_score_from_top(&grid, row, col)
                    * view_score_from_left(&grid, row, col)
                    * view_score_from_right(&grid, row, col)
                    * view_score_from_bottom(&grid, row, col);
                view_score.push(score);
            }
        }

        assert_eq!(view_score.into_iter().max(), Some(8));
    }
}
