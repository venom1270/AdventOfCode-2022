use std::cmp;

use advent_of_code_2022::file_utils::read_lines;

/*struct Directory {
    name: String,
    path: String,
    size: u32, // Files size only
    parent: Option<Box<Directory>>,
    files: Vec<(String, u32)>,
    directories: Vec<(String, Directory)>
}
impl Directory {
    pub fn new(name: String, path: String, parent: Option<Box<Directory>>) -> Self {
        Directory {
            name,
            path,
            size: 0,
            parent,
            files: Vec::new(),
            directories: Vec::new()
        }
    }

    pub fn add_file(&mut self, name: String, size: u32) {
        self.size += size;
        self.files.push((name, size));
    }

    pub fn add_directory(&mut self, dir: Directory) {
        self.directories.push((
            dir.name.to_string(), 
            dir
            //Directory::new(name.to_string(), self.path.to_string() + "/" + &name, Option::Some(Box::new(*self)))
            //Directory::new(name.to_string(), self.path.to_string() + "/" + &name, Option::Some(self.get_self()))
        ));
    }*/

    /*fn get_self(self) -> Box<Self> {
        Box::new(self)
    }

    fn create_directory(&self, name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            path: format!("{}/{}", self.path, name),
            size: 0,
            parent: Option::Some(Box::new(*self)),
            files: Vec::new(),
            directories: Vec::new()
        }
    }*/

//}


struct Directory {
    name: String,
    path: String,
    size: u32, // Files size only
    id: i32,
    parent: Option<i32>,
    files: Vec<(String, u32)>,
    directories: Vec<(String, i32)>
}
impl Directory {
    pub fn new(name: String, path: String, id: i32, parent: i32) -> Self {
        Directory {
            name,
            path,
            size: 0,
            id,
            parent: Some(parent),
            files: Vec::new(),
            directories: Vec::new()
        }
    }

    pub fn add_file(&mut self, name: String, size: u32) {
        self.size += size;
        self.files.push((name, size));
    }

    pub fn add_directory2(&mut self, dir: &Directory) {
        self.directories.push((
            dir.name.to_string(), 
            dir.id
            //Directory::new(name.to_string(), self.path.to_string() + "/" + &name, Option::Some(Box::new(*self)))
            //Directory::new(name.to_string(), self.path.to_string() + "/" + &name, Option::Some(self.get_self()))
        ));
    }

    pub fn add_directory(&mut self, name: String, id: i32) {
        self.directories.push((
            name.to_string(), 
            id
        ));
    }
}



struct FileSystem {
    directories: Vec<Directory>,
    current_path: String,
    current_dir: i32,
}
impl FileSystem {
    pub fn new() -> Self {
        FileSystem { 
            directories: Vec::new(), 
            current_path: "".to_string(), 
            current_dir: -1 
        }
    }

    pub fn add_directory(&mut self, name: &str) {
        let id = self.directories.len() as i32;
        let mut path = "".to_string();
        if name.eq("/") {
            path = "/".to_string();
        } else if self.current_path.eq("/") {
            path = format!("/{}", name);
        } else {
            path = format!("{}/{}", self.current_path, name);
        }
        //println!("PATH: {} {}", path, self.current_dir);
        let new_dir = Directory::new(name.to_string(), path, id, self.current_dir);
        let id = self.directories.len() as i32;
        self.directories.push(new_dir);
        let curr_dir = self.get_directory_mut();
        //curr_dir.add_directory(name.to_string(), id);        
        if let Some(cd) = curr_dir {
            cd.add_directory(name.to_string(), id);
        }
    }

    pub fn add_file(&mut self, name: &str, size: u32) {
        // https://stackoverflow.com/questions/43550632/how-can-i-change-fields-of-elements-in-vectors
        let tmp = self.directories.get_mut(self.current_dir as usize).unwrap();
        tmp.add_file(name.to_string(), size);
    }



    pub fn move_to_dir(&mut self, name: &str) {
        // TODO: hashmap
        /*let found = self.directories.into_iter()
                    .filter(|d| d.parent.unwrap() == self.current_dir && d.name.eq(name))
                    .collect::<Vec<Directory>>().first();*/
        let mut dir: Option<(&str, i32)> = None;
        for d in &self.directories {
            //println!("name: {}  parent: {} || {} {}", d.name, d.parent.unwrap(), name, self.current_dir);
            if d.name.eq(name) && d.parent.unwrap() == self.current_dir {
                dir = Some((&d.path, d.id));
                break;
            }
        }
        match dir {
            Some((path, id)) => {
                self.current_dir = id; 
                self.current_path = String::from(path);
            },
            None => panic!("Error directory not found")
        }
    }

    pub fn move_back(&mut self) {
        let tmp = self.directories.get(self.current_dir as usize).unwrap();
        // Set current dir to one up/back
        let up = tmp.parent.unwrap();
        self.current_dir = up;
        // Update path
        let tmp = self.directories.get(self.current_dir as usize).unwrap();
        self.current_path = tmp.path.clone();
    }

    pub fn print_current_dir(&self) {
        println!("{} | {}", self.current_dir, self.current_path);
    }

    pub fn to_root(&mut self) {
        self.current_dir = 0;
        self.current_path = "/".to_string();
    }

    pub fn get_directory(&self) -> &Directory {
        self.directories.get(self.current_dir as usize).unwrap()
    }

    pub fn get_directory_mut(&mut self) -> Option<&mut Directory> {
        //self.directories.get_mut(self.current_dir as usize).unwrap()
        self.directories.get_mut(self.current_dir as usize)
    }

    pub fn get_directory_at_index(&self, index: i32) -> &Directory {
        self.directories.get(index as usize).unwrap()
    }

}

fn parse_input(file_path: &str) -> FileSystem{

    let mut fs: FileSystem = FileSystem::new();

    if let Ok(lines) = read_lines(file_path) {

        fs.add_directory("/");
        fs.move_to_dir("/");
        fs.print_current_dir();

        for line in lines.skip(1) {
            let line = line.expect("Parse error");
            
            if line.starts_with("$") {
                let mut s = line.split(' ').fuse();
                s.next();
                if s.next().expect("Error parsing $ command").eq("cd") {
                    let dir = s.next().expect("Error parsing $ cd dir");
                    if dir.eq("..") {
                        fs.move_back();
                    } else {
                        fs.move_to_dir(dir);
                    }
                }
            } else {
                let mut s = line.split(' ').fuse();
                match s.next().expect("Error parsing") {
                    "dir" => {
                        let dir_name = s.next().expect("Error parsing dir");
                        fs.add_directory(dir_name);
                    },
                    size => {
                        let file_name = s.next().expect("Error parsing file");
                        let size: u32 = size.parse().expect("Error parsing file size");
                        fs.add_file(file_name, size)
                    }
                }
            }
        }



    }

    fs

}

fn calculate_size(dir: &Directory, fs: &FileSystem) -> (u32, u32) {
    let mut size = dir.size;
    let mut result = 0;
    if dir.directories.len() > 0 {
        for d in &dir.directories {
            let (s, r) = calculate_size(fs.get_directory_at_index(d.1), fs);
            size += s;
            result += r;
        }
    }
    if size <= 100000 {
        result += size;
    }
    //println!("{} -- {}", size, result);
    return (size, result);
}

fn calculate_freed_space(FULL_SIZE: u32, dir: &Directory, fs: &FileSystem) -> (u32, u32) {
    let mut size = dir.size;
    let mut freed_space = 999999999;
    if dir.directories.len() > 0 {
        for d in &dir.directories {
            let (s, frd_spc) = calculate_freed_space(FULL_SIZE, fs.get_directory_at_index(d.1), fs);
            size += s;
            freed_space = cmp::min(freed_space, frd_spc);
        }
    }
    if 70000000 - FULL_SIZE + size >= 30000000 {
        freed_space = cmp::min(freed_space, size);
    }
    //println!("{} -- {}", size, result);
    return (size, freed_space);
}

pub fn part1and2() {
    //let mut fs = parse_input("src/day7/test.txt");
    let mut fs = parse_input("src/day7/1.txt");
    fs.print_current_dir();
    fs.to_root();
    fs.print_current_dir();
    let (full_size, result) = calculate_size(fs.get_directory(), &fs);
    println!("PART 1 | Full size: {} | Sum of all with size <100000 {}", full_size, result);
    fs.to_root();
    let (full_size, freed_space) = calculate_freed_space(full_size, fs.get_directory(), &fs);
    println!("PART 2 | Full size: {} | Space freed after deleting smalles folder possible: {}", full_size, freed_space);

}



