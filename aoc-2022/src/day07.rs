use std::collections::HashSet;
use std::thread::current;
use regex::Regex;

struct File {
    name: String,
    size: i32
}

struct Folder<'a> {
    name: String,
    parent: Option<&'a Folder<'a>>,
    folders: Vec<Folder<'a>>,
    files: Vec<File>,
    total_size: i32
}

pub fn generator(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &Vec<&str>) -> i32 {
    let mut root = Folder {
        name: "/".to_string(),
        parent: None,
        folders: Default::default(),
        files: Default::default(),
        total_size: 0
    };
    
    let mut current_folder = &mut root;
    
    for line in input.iter() {
        let captures = Regex::new(r"(\$ \w{2}|dir|\d+) *(.*)").unwrap().captures(&line).unwrap();
        match &captures[1] {
            "dir" => {
                let directory_name = &captures[2];

                let folder = Folder {
                    name: String::from(directory_name),
                    parent: Some(&current_folder),
                    folders: Vec::new(),
                    files: Vec::new(),
                    total_size: 0,
                };
                
                let mut folders = &mut current_folder.folders;

                folders.push(folder);
            }
            "$ cd" => {
                let target_directory = &captures[2];
                match target_directory {
                    "/" => {
                        current_folder = &mut root;
                    }
                    ".." => {
                        current_folder = &mut current_folder.parent.unwrap();
                    }
                    _ => {
                        current_folder = &mut current_folder.folders.into_iter()
                            .find(|x| x.name == target_directory).unwrap();
                    }
                }
            }
            "$ ls" => {}
            _ => {
                let file_name = &captures[2];
                let file_size = &captures[1].parse::<i32>().unwrap();

                let mut files = &current_folder.files;
                files.push(File {
                    name: file_name.to_string(),
                    size: *file_size
                })
            }
        }
    }
    0
}

pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // assert_eq!(part1("input/2022/day7.txt"), 0);
        part1(&Vec::new());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("input/2022/day7.txt"), 0);
    }
}