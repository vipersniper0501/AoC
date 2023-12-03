// Note: I heavily relied on the attached video to solve this problem as I
// couldn't quite understand or figure out how to build and use a tree in Rust.
// Video: https://www.youtube.com/watch?v=ajf7DfqeIQY



use std::{fs, io::{self, BufRead}, collections::HashMap, rc::Rc, cell::RefCell};

#[derive(Default)]
struct Dir {
    _name: String,
    size: RefCell<usize>,
    // Children
    sub_dir: RefCell<HashMap<String, Rc<Dir>>>,
    // Parent
    parent: Option<Rc<Dir>>
}

impl Dir {
    fn new() -> Self{
        Self::default()
    }

    fn get_size(&self) -> usize {
        // So confused as to what is going on here...
        *self.size.borrow()
            + self
                .sub_dir
                .borrow()
                .values()
                .fold(0, |a, b| a + b .get_size())
    }
}

fn main() {
    let file = fs::File::open("data/input")
        .expect("No File Found");
    let reader = io::BufReader::new(file);

    let root = Rc::new(Dir::new());

    let mut cwd = Rc::clone(&root);

    // Build File Tree
    for line in reader.lines() {

        let mut line_data = String::new();
        match line {
            Ok(v) => line_data = v,
            Err(e) => println!("{e}")
        }

        let commands = line_data.split(" ").collect::<Vec<&str>>();
        match (commands[0], commands[1]) {
            ("$", "ls") => {},
            ("$", "cd") => match commands[2] {
                "/" => cwd = Rc::clone(&root),
                ".." => cwd = Rc::clone(&cwd.parent.as_ref().unwrap()),
                dirname => {
                    let newdir = cwd.sub_dir.borrow().get(dirname).unwrap().clone();
                    cwd = newdir;
                }
            },
            ("dir", dirname) => {
                cwd.sub_dir.borrow_mut().insert(
                    dirname.to_string(),
                    Rc::new(Dir { 
                        _name: dirname.to_string(), 
                        size: RefCell::new(0), 
                        sub_dir: RefCell::new(HashMap::new()), 
                        parent: Some(Rc::clone(&cwd)) 
                    }),
                );
            }
            (size, _name) => {
                *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
            },
        
        }
    }

    // Part 1

    let mut to_visit = vec![Rc::clone(&root)];
    let mut total = 0;

    while let Some(dir) = to_visit.pop() {
        for d in dir.sub_dir.borrow().values() {
            to_visit.push(Rc::clone(d));
        }
        let size = dir.get_size();
        if size <= 100000 {
            total += size;
        }
    }

    // Part 2
    
    let total_size = root.get_size();
    let free_space = 70000000 - total_size;
    let space_needed = 30000000 - free_space;

    let mut to_visit = vec![Rc::clone(&root)];
    let mut best = usize::MAX;

    
    while let Some(dir) = to_visit.pop() {
        for d in dir.sub_dir.borrow().values() {
            to_visit.push(Rc::clone(d));
        }
        let size = dir.get_size();
        if size >= space_needed {
            best = best.min(size);
        }
    }

    println!("Part 1: total size of file system: {}", total);
    println!("Part 2: total size of directory to be deleted: {}", best);
}

