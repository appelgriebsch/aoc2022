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

    pub fn find_subdirectory(&self, name: &str) -> Option<&DirectoryMetadata> {
        if let Some(Node::Directory(directory)) = self.items.iter().find(|item| match item {
            Node::Directory(d) => d.name() == name,
            _ => false,
        }) {
            return Some(directory);
        }

        None
    }

    pub fn find_subdirectories<F: Fn(&DirectoryMetadata) -> bool>(
        &self,
        f: F,
    ) -> Vec<&DirectoryMetadata> {
        self.items
            .iter()
            .filter(|item| match item {
                Node::Directory(d) => f(d),
                _ => false,
            })
            .filter_map(|item| match item {
                Node::Directory(d) => Some(d),
                _ => None,
            })
            .collect::<_>()
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
        let small_dirs = root_dir.find_subdirectories(|dir| dir.size() < 100000);
        dbg!(&small_dirs);
        assert_eq!(small_dirs.len(), 1);
        assert_eq!(small_dirs.get(0).unwrap().name(), "a");
    }

    #[test]
    fn processes_sample_part2() {}
}
