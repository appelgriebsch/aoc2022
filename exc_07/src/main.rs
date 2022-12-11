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

    pub fn size(&self) -> usize {
        self.items
            .iter()
            .map(|node| match node {
                Node::File(f) => f.size(),
                Node::Directory(d) => d.size(),
            })
            .sum()
    }

    pub fn parse(&mut self, items: &[&str]) -> usize {
        lazy_static! {
            static ref RE_CMD: Regex =
                Regex::new(r"^\$\s(?P<cmd>\S+)(\s(?P<arg>\S+))?$").expect("regex is wrong!");
            static ref RE_FILE: Regex =
                Regex::new(r"^(?P<size>\d+)\s(?P<name>\S+)$").expect("regex is wrong!");
        }

        let mut no_of_items_read = 0;
        let mut no_of_items_to_advance = 0;
        for (i, item) in items.iter().enumerate() {
            if no_of_items_to_advance > 0 {
                no_of_items_to_advance -= 1;
                continue;
            }
            if RE_FILE.is_match(item) {
                if let Some(captures) = RE_FILE.captures(item) {
                    let size: usize = captures["size"].parse().unwrap();
                    let name: &str = captures["name"].as_ref();
                    println!(
                        "Found file {name} with size {size} in directory {}",
                        self.name()
                    );
                    self.items.push(Node::File(FileMetadata::new(name, size)));
                }
            }
            if RE_CMD.is_match(item) {
                if let Some(captures) = RE_CMD.captures(item) {
                    if let Some(arg) = captures.name("arg") {
                        match (captures["cmd"].as_ref(), arg.as_str()) {
                            ("cd", "..") => {
                                no_of_items_read = i + 1;
                                break;
                            }
                            ("cd", subdir) => {
                                let mut subdir_node = DirectoryMetadata::new(subdir);
                                no_of_items_to_advance = subdir_node.parse(&items[(i + 1)..]);
                                self.items.push(Node::Directory(subdir_node));
                            }
                            _ => continue,
                        }
                    }
                }
            }

            no_of_items_read = i + 1;
        }

        println!(
            "Processed directory {} with size {}",
            self.name(),
            self.size()
        );
        no_of_items_read
    }
}

#[derive(Debug)]
pub enum Node {
    File(FileMetadata),
    Directory(DirectoryMetadata),
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
    use crate::DirectoryMetadata;

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
        root_dir.parse(&lines);
        assert_eq!(root_dir.name(), "/");
        assert_eq!(root_dir.size(), 48381165);
        assert_eq!(root_dir.find_subdirectory("a").unwrap().size(), 94853);
        assert_eq!(root_dir.find_subdirectory("d").unwrap().size(), 24933642);
    }

    #[test]
    fn processes_sample_part2() {}
}
