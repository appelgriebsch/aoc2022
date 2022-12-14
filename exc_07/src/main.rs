use lazy_static::lazy_static;
use regex::Regex;
use std::{env, fs::File, io::Read, path::Path};

#[derive(Debug)]
pub struct FileMetadata {
    name: String,
    size: usize,
}

impl FileMetadata {
    pub fn new(name: &str, size: usize) -> Self {
        FileMetadata {
            name: name.to_owned(),
            size,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
pub struct DirectoryMetadata {
    name: String,
    items: Vec<Node>,
}

impl DirectoryMetadata {
    pub fn new(name: &str) -> Self {
        DirectoryMetadata {
            name: name.to_owned(),
            items: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn items(&self) -> &[Node] {
        self.items.as_ref()
    }

    pub fn directories(&self) -> Vec<&DirectoryMetadata> {
        self.items
            .iter()
            .filter_map(|item| match item {
                Node::Directory(d) => {
                    let result = [vec![d], d.directories()].concat();
                    Some(result)
                }
                _ => None,
            })
            .flatten()
            .collect()
    }

    pub fn find_subdirectory(&self, name: &str) -> Option<&DirectoryMetadata> {
        if let Some(Node::Directory(directory)) = self.items.iter().find(|item| match item {
            Node::Directory(d) => d.name() == name,
            _ => false,
        }) {
            return Some(directory);
        }

        None
    }

    pub fn size(&self) -> usize {
        self.items
            .iter()
            .map(|node| match node {
                Node::File(f) => f.size(),
                Node::Directory(d) => d.size(),
            })
            .sum()
    }

    pub fn parse(&mut self, parser: &mut Parser) {
        loop {
            match parser.next() {
                Some(ParseResult::Unknown) => continue,
                Some(ParseResult::File(f)) => self.items.push(Node::File(f)),
                Some(ParseResult::EnterDirectory(name)) => {
                    let mut subdir = DirectoryMetadata::new(name);
                    subdir.parse(parser);
                    self.items.push(Node::Directory(subdir));
                }
                _ => break,
            }
        }
    }
}

#[derive(Debug)]
pub enum Node {
    File(FileMetadata),
    Directory(DirectoryMetadata),
}

pub enum ParseResult<'a> {
    Unknown,
    EnterDirectory(&'a str),
    LeaveDirectory,
    File(FileMetadata),
}

pub struct Parser<'a> {
    items: Vec<&'a str>,
    current_index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(items: Vec<&'a str>) -> Self {
        Parser {
            items,
            current_index: 0,
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = ParseResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        lazy_static! {
            static ref RE_CMD: Regex =
                Regex::new(r"^\$\s(?P<cmd>\S+)(\s(?P<arg>\S+))?$").expect("regex is wrong!");
            static ref RE_FILE: Regex =
                Regex::new(r"^(?P<size>\d+)\s(?P<name>\S+)$").expect("regex is wrong!");
        }
        if let Some(item) = self.items.get(self.current_index) {
            self.current_index += 1;
            if RE_FILE.is_match(item) {
                if let Some(captures) = RE_FILE.captures(item) {
                    let size: usize = captures["size"].parse().unwrap();
                    let name: &str = captures["name"].as_ref();
                    Some(ParseResult::File(FileMetadata::new(name, size)))
                } else {
                    Some(ParseResult::Unknown)
                }
            } else if RE_CMD.is_match(item) {
                if let Some(captures) = RE_CMD.captures(item) {
                    if let Some(arg) = captures.name("arg") {
                        match (captures["cmd"].as_ref(), arg.as_str()) {
                            ("cd", "..") => Some(ParseResult::LeaveDirectory),
                            ("cd", subdir) => Some(ParseResult::EnterDirectory(subdir)),
                            _ => Some(ParseResult::Unknown),
                        }
                    } else {
                        Some(ParseResult::Unknown)
                    }
                } else {
                    Some(ParseResult::Unknown)
                }
            } else {
                Some(ParseResult::Unknown)
            }
        } else {
            None
        }
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

        let mut root_dir = DirectoryMetadata::new("/");
        let lines: Vec<_> = input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.trim())
            .skip(1)
            .collect();
        let mut parser = Parser::new(lines);
        root_dir.parse(&mut parser);

        let dirs = root_dir.directories();
        let no_of_dirs = dirs.len();
        let dir_size: usize = dirs
            .into_iter()
            .filter(|dir| dir.size() < 100000)
            .map(|dir| dir.size())
            .sum();
        println!(
            "Scanned {no_of_dirs} directories, and found with at most 100000 bytes: {dir_size}"
        );

        let free_space = 70000000 - root_dir.size();
        let required_space = 30000000 - free_space;
        println!("{free_space} left on device, but need {required_space} more to update.");

        let mut dirs = root_dir.directories();
        dirs.sort_by_key(|a| a.size());
        let remove_dir = dirs
            .into_iter()
            .filter(|dir| dir.size() >= required_space)
            .take(1)
            .next()
            .expect("No matching directory found to clear up enough space on device");

        print!(
            "Found directory {} with size {} to ease required amount of space",
            remove_dir.name(),
            remove_dir.size()
        );
    }
}

#[cfg(test)]
mod test {
    use crate::{DirectoryMetadata, Parser};

    #[test]
    fn processes_sample_part1() {
        let input = r#"$ cd /
                       $ ls
                       dir a
                       14848514 b.txt
                       8504156 c.dat
                       dir d
                       $ cd a
                       $ ls
                       dir e
                       29116 f
                       2557 g
                       62596 h.lst
                       $ cd e
                       $ ls
                       584 i
                       $ cd ..
                       $ cd ..
                       $ cd d
                       $ ls
                       4060174 j
                       8033020 d.log
                       5626152 d.ext
                       7214296 k"#;

        let mut root_dir = DirectoryMetadata::new("/");
        let lines: Vec<_> = input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.trim())
            .skip(1)
            .collect();
        let mut parser = Parser::new(lines);
        root_dir.parse(&mut parser);
        dbg!(&root_dir);
        assert_eq!(root_dir.name(), "/");
        assert_eq!(root_dir.size(), 48381165);
        assert_eq!(root_dir.find_subdirectory("a").unwrap().size(), 94853);
        assert_eq!(root_dir.find_subdirectory("d").unwrap().size(), 24933642);

        let dirs = root_dir.directories();
        assert_eq!(dirs.len(), 3);
        let dir_size: usize = dirs
            .into_iter()
            .filter(|dir| dir.size() < 100000)
            .map(|dir| dir.size())
            .sum();
        assert_eq!(dir_size, 95437);
    }

    #[test]
    fn processes_sample_part2() {
        let input = r#"$ cd /
                       $ ls
                       dir a
                       14848514 b.txt
                       8504156 c.dat
                       dir d
                       $ cd a
                       $ ls
                       dir e
                       29116 f
                       2557 g
                       62596 h.lst
                       $ cd e
                       $ ls
                       584 i
                       $ cd ..
                       $ cd ..
                       $ cd d
                       $ ls
                       4060174 j
                       8033020 d.log
                       5626152 d.ext
                       7214296 k"#;

        let mut root_dir = DirectoryMetadata::new("/");
        let lines: Vec<_> = input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.trim())
            .skip(1)
            .collect();
        let mut parser = Parser::new(lines);
        root_dir.parse(&mut parser);

        let free_space = 70000000 - root_dir.size();
        let required_space = 30000000 - free_space;

        let mut dirs = root_dir.directories();
        assert_eq!(dirs.len(), 3);
        dirs.sort_by_key(|a| a.size());
        let remove_dirs: Vec<_> = dirs
            .into_iter()
            .filter(|dir| dir.size() >= required_space)
            .take(1)
            .collect();
        assert_eq!(remove_dirs.len(), 1);
        assert_eq!(remove_dirs.get(0).unwrap().name(), "d");
        assert_eq!(remove_dirs.get(0).unwrap().size(), 24933642);
    }
}
