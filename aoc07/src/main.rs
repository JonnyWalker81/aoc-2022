use std::{
    collections::HashMap,
    io::{self, Read},
};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;

    Ok(())
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
    Dir(String),
    File(String, i32),
}

impl From<&str> for Command {
    fn from(item: &str) -> Self {
        let parts: Vec<&str> = item.split_whitespace().collect();
        // println!("{:?}", parts);
        if parts.len() == 3 && parts[0] == "$" {
            // cd
            Self::Cd(parts[2].to_string())
        } else if parts.len() == 2 && parts[0] == "$" {
            Self::Ls
        } else {
            // file or dir
            if parts[0] == "dir" {
                Self::Dir(parts[1].to_string())
            } else {
                Self::File(parts[1].to_string(), parts[0].parse().unwrap())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum FilesystemItem {
    File(String, i32),
    Dir(String, Vec<FilesystemItem>),
}

#[derive(Debug, Clone)]
struct Filesystem {
    root: HashMap<String, Vec<FilesystemItem>>,
    current_dir: Vec<String>,
}

impl Filesystem {
    fn new() -> Self {
        Self {
            root: HashMap::new(),
            current_dir: vec![],
        }
    }

    fn current_dir_str(&self) -> String {
        format!("{}", self.current_dir.join("/"))
    }

    fn push_file(&mut self, filename: &str, size: i32) {
        let dir = self.current_dir_str();
        if let Some(ref mut children) = self.root.get_mut(&dir) {
            children.push(FilesystemItem::File(filename.to_string(), size));
        }
    }

    fn push_dir(&mut self, dir_name: &str) {
        let dir = self.current_dir_str();
        if let Some(ref mut children) = self.root.get_mut(&dir) {
            children.push(FilesystemItem::Dir(dir_name.to_string(), vec![]));
        }
    }

    fn set_current_dir(&mut self, dir_name: &str) {
        if dir_name == ".." {
            self.current_dir.pop();
        } else if dir_name == "/" {
            self.current_dir.push("".to_string());
            self.root.entry("".to_string()).or_insert(vec![]);
        } else {
            self.current_dir.push(dir_name.to_string());
            self.root.entry(self.current_dir_str()).or_insert(vec![]);
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let commands: Vec<Command> = input.lines().map(|l| l.into()).collect();

    let mut filesystem = Filesystem::new();

    for c in commands {
        match c {
            Command::Cd(d) => filesystem.set_current_dir(&d),
            Command::Ls => {}
            Command::Dir(d) => filesystem.push_dir(&d),
            Command::File(f, s) => filesystem.push_file(&f, s),
        }
    }

    let mut sizes: HashMap<String, usize> = HashMap::new();
    for (path, files) in filesystem.root.iter() {
        let dirs: Vec<&str> = path.split("/").collect();
        let size = files
            .iter()
            .map(|f| {
                if let FilesystemItem::File(_, size) = f {
                    *size as usize
                } else {
                    0
                }
            })
            .sum();

        for i in 0..dirs.len() {
            sizes
                .entry((&dirs[0..=i]).iter().cloned().collect::<String>())
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }

    let sum = sizes
        .iter()
        .filter(|(_, v)| **v < 100_000)
        .map(|(_, v)| v)
        .sum::<usize>();

    println!("{:#?}", sum);

    let outtermost = sizes.get("").unwrap();
    let free_space = 70_000_000 - outtermost;
    let needed_free_space = 30_000_000 - free_space;

    let mut used_sizes: Vec<usize> = sizes
        .iter()
        .map(|(_, &v)| v)
        .filter(|s| *s > needed_free_space)
        .collect();

    used_sizes.sort();

    println!("{}", used_sizes[0]);

    Ok(())
}
