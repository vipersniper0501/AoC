use std::{fs, io::{self, BufRead}, collections::HashSet};

#[derive(Default)]
struct Day9 {
    steps: Vec<(Direction, i32)>,
}

impl Day9 {

    fn new() -> Self {
        Self::default()
    }

    fn parse(&mut self) {
        let file = fs::File::open("data/input")
            .expect("No File Found");

        let reader = io::BufReader::new(file);
        
        for line in reader.lines() {
            let mut line_data = String::new();
            match line {
                Ok(v) => line_data = v,
                Err(e) => println!("{e}")
            }

            let (dir, dist) = line_data.split_once(' ').unwrap();
            let dir = Direction::parse(dir);
            let dist = dist.parse().unwrap();
            self.steps.push((dir, dist));

        }
    }

    fn part1(&self) {

        let mut snake = Snake::new(2);

        for (dir, amount) in &self.steps {
            for _ in 0..*amount {
                snake.make_move(dir);
            }
        }
        println!("Part1: {}", snake.visited.len());
    }

    fn part2(&self) {
        let mut snake = Snake::new(10);

        for (dir, amount) in &self.steps {
            for _ in 0..*amount {
                snake.make_move(dir);
            }
        }
        println!("Part2: {}", snake.visited.len());
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn parse(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("invalid direction '{s}'"),
        }
    }
}

#[derive(Default)]
struct Snake {
    seg: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>
}

impl Snake {
    const DIR: [(i32,i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn new(len: usize) -> Self {
        Self { 
            seg: vec![(0,0); len], 
            visited: HashSet::new() 
        }
    }

    fn make_move(&mut self, dir: &Direction) {
        let delta = Self::DIR[*dir as usize];
        self.seg[0].0 += delta.0;
        self.seg[0].1 += delta.1;

        for i in 1..self.seg.len() {

        
            let rowdiff = self.seg[i - 1].0 - self.seg[i].0;
            let coldiff = self.seg[i - 1].1 - self.seg[i].1;

            if rowdiff == 0 && coldiff.abs() > 1 {
                self.seg[i].1 += coldiff.signum();
            } else if coldiff == 0 && rowdiff.abs() > 1 {
                self.seg[i].0 += rowdiff.signum();
            } else if rowdiff.abs() > 1 || coldiff.abs() > 1 {
                self.seg[i].0 += rowdiff.signum();
                self.seg[i].1 += coldiff.signum();
            }
        }
        self.visited.insert(self.seg[self.seg.len() - 1]);
    }
}

fn main() {
    let mut day = Day9::new();
    day.parse();
    day.part1();
    day.part2();
}
