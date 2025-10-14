#[derive(Debug)]
struct Dir {
    name: String,
    parent: std::path::PathBuf,
    files: Vec<File>,
    dirs: Vec<std::path::PathBuf>,
    size: usize,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct FileSystem {
    path: std::path::PathBuf,
    dirs: std::collections::HashMap<std::path::PathBuf, Dir>,
}

impl FileSystem {
    fn new() -> FileSystem {
        let root = Dir {
            name: "/".to_owned(),
            parent: std::path::PathBuf::new(),
            files: vec![],
            dirs: vec![],
            size: 0,
        };
        let path = std::path::PathBuf::from("/");

        let dirs = std::collections::HashMap::from([(path.clone(), root)]);

        FileSystem { path, dirs }
    }

    fn create_dir(&mut self, name: &str) {
        let dir = Dir {
            name: name.to_owned(),
            parent: self.path.clone(),
            files: vec![],
            dirs: vec![],
            size: 0,
        };

        let mut dir_path = self.path.clone();
        dir_path.push(dir.name.clone());
        self.dirs
            .get_mut(&self.path)
            .unwrap()
            .dirs
            .push(dir_path.clone());
        self.dirs.insert(dir_path, dir);
    }

    fn create_file(&mut self, name: &str, size: usize) {
        let file = File {
            name: name.to_owned(),
            size: size,
        };

        self.dirs.get_mut(&self.path).unwrap().files.push(file);
        let mut size_update_queue = std::vec![self.path.clone()];
        while let Some(dir_path) = size_update_queue.pop() {
            if dir_path.iter().count() == 0 {
                continue;
            }
            let dir = self.dirs.get_mut(&dir_path).unwrap();
            dir.size += size;
            size_update_queue.push(dir.parent.clone());
        }
    }

    fn cd(&mut self, arg: &str) {
        match arg {
            ".." => {
                self.path.pop();
            }
            "/" => self.path = std::path::PathBuf::from("/"),
            _ => self.path.push(arg),
        }
    }

    #[allow(unused)]
    fn print(&self) {
        let mut queue = std::vec![std::path::PathBuf::from("/")];
        while let Some(path) = queue.pop() {
            let dir = self.dirs.get(&path).unwrap();
            let i = path.iter().count();
            println!("{:i$}- {} (dir, size={})", "", dir.name, dir.size);
            let i = i + 1;
            for file in dir.files.iter() {
                println!("{:i$}- {} (file, size={})", "", file.name, file.size);
            }
            let mut dirs = dir.dirs.clone();
            dirs.sort();
            queue.extend(dirs.into_iter());
        }
    }
}

pub fn part01(file_path: &str) -> usize {
    let puzzle_input: String = std::fs::read_to_string(file_path).unwrap();
    let mut terminal_output = puzzle_input.lines();

    let mut file_system = FileSystem::new();
    while let Some(output) = terminal_output.next() {
        if output.starts_with("$") {
            let output = output.strip_prefix("$ ").unwrap();
            if output.starts_with("cd") {
                let arg = output.split_once(' ').unwrap().1;
                file_system.cd(arg);
            }
        } else {
            if output.starts_with("dir") {
                file_system.create_dir(output.split_once(' ').unwrap().1);
            } else {
                let (size, name) = output.split_once(' ').unwrap();
                file_system.create_file(name, size.parse().unwrap());
            }
        }
    }

    let total_sizes = file_system
        .dirs
        .values()
        .map(|d| d.size)
        .filter(|&x| x <= 100000)
        .sum();

    total_sizes
}
