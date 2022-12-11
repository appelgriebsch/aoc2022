use std::{collections::HashSet, env, fs::File, io::Read, path::Path};

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

    if let Some(marker_pos) = calculate_marker(&input, 4) {
        println!("Found signal marker pos at {marker_pos}");
    } else {
        println!("Input doesn't have a signal marker");
    }

    if let Some(marker_pos) = calculate_marker(&input, 14) {
        println!("Found message marker pos at {marker_pos}");
    } else {
        println!("Input doesn't have a message marker");
    }
}

fn calculate_marker(input: &str, window_size: usize) -> Option<usize> {
    let chars: Vec<char> = input.chars().collect();
    let iter = chars.windows(window_size);
    for (counter, batch) in iter.enumerate() {
        let mut uniq = HashSet::new();
        if batch.iter().all(move |char| uniq.insert(char)) {
            return Some(counter + batch.len());
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::calculate_marker;

    #[test]
    fn find_marker_sample1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = calculate_marker(input, 4);
        assert_eq!(result, Some(7));
    }

    #[test]
    fn find_marker_sample2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = calculate_marker(input, 4);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn find_marker_sample3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = calculate_marker(input, 4);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn find_marker_sample4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = calculate_marker(input, 4);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn find_marker_sample5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = calculate_marker(input, 4);
        assert_eq!(result, Some(11));
    }

    #[test]
    fn find_message_sample1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let result = calculate_marker(input, 14);
        assert_eq!(result, Some(19));
    }

    #[test]
    fn find_message_sample2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let result = calculate_marker(input, 14);
        assert_eq!(result, Some(23));
    }

    #[test]
    fn find_message_sample3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let result = calculate_marker(input, 14);
        assert_eq!(result, Some(23));
    }

    #[test]
    fn find_message_sample4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let result = calculate_marker(input, 14);
        assert_eq!(result, Some(29));
    }

    #[test]
    fn find_message_sample5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let result = calculate_marker(input, 14);
        assert_eq!(result, Some(26));
    }
}
