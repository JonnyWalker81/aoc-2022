use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    // part2(&input)?;

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

#[derive(Debug, Clone)]
enum FilesystemItem {
    File(String, i32),
    Dir(String, Vec<FilesystemItem>),
}

#[derive(Debug, Clone)]
struct Filesystem {
    // root: Vec<FilesystemItem>,
    current_dir: Vec<FilesystemItem>,
    level: i32,
}

impl Filesystem {
    fn new() -> Self {
        Self {
            // root: vec![],
            current_dir: vec![],
            level: -1,
        }
    }

    fn push_file(&mut self, filename: &str, size: i32) {
        println!("File Level: {}", self.level);
        if let Some(FilesystemItem::Dir(_, ref mut d)) =
            self.current_dir.get_mut(self.level as usize)
        {
            d.push(FilesystemItem::File(filename.to_string(), size));
        }
    }

    fn push_dir(&mut self, dir_name: &str) {
        println!("Dir Level: {}", self.level);
        if let Some(FilesystemItem::Dir(_, ref mut d)) =
            self.current_dir.get_mut(self.level as usize)
        {
            d.push(FilesystemItem::Dir(dir_name.to_string(), vec![]));
        }
    }

    fn set_current_dir(&mut self, dir_name: &str) {
        if dir_name == ".." {
            self.level -= 1;
        } else if dir_name == "/" {
            self.level = 0;
            // if self.current_dir.len() > 1 {
            //     self.current_dir = self.current_dir[1..].to_vec();
            // }
        } else {
            self.current_dir
                .push(FilesystemItem::Dir(dir_name.to_string(), vec![]));
            self.level += 1;
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let commands: Vec<Command> = input.lines().map(|l| l.into()).collect();

    let mut filesystem = Filesystem::new();

    for c in commands {
        println!("{:?}", c);
        match c {
            Command::Cd(d) => filesystem.set_current_dir(&d),
            Command::Ls => {}
            Command::Dir(d) => filesystem.push_dir(&d),
            Command::File(f, s) => filesystem.push_file(&f, s),
        }
    }

    println!("{:#?}", filesystem);
    Ok(())
}
