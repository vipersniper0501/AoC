use std::{fs, io::{self, BufRead}};

struct Day8 {
    tree_map: Vec<Vec<i32>>,

}

impl Day8 {

    fn new() -> Self {
        Self {
            tree_map: Vec::new(),
        }
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

            self.tree_map.push(line_data.chars().map(|element| 
                    element.to_string().parse::<i32>().unwrap()).collect());
        }
    }

    fn part1(&self) {
        let mut visible_trees = 0;
        for row in 0..self.tree_map.len() {
            for col in 0..self.tree_map[row].len() {
                let mut inc_flag = false;
                let current_val = self.tree_map[row][col];
                {
                    let mut vis_flag = true;
                    for hl in (0..col).rev() {
                        if self.tree_map[row][hl] >= current_val {
                            vis_flag = false;
                        }
                    }
                    if vis_flag == true {
                        inc_flag = true;
                    }
                }
                if inc_flag != true {
                    let mut vis_flag = true;
                    for hr in col+1..self.tree_map[row].len() {
                        if self.tree_map[row][hr] >= current_val {
                            vis_flag = false;
                        }
                    }
                    if vis_flag == true {
                        inc_flag = true;
                    }
                }
                if inc_flag != true {
                    let mut vis_flag = true;
                    for vt in (0..row).rev() {
                        if self.tree_map[vt][col] >= current_val {
                            vis_flag = false;
                        }
                    }
                    if vis_flag == true {
                        inc_flag = true;
                    }
                }
                if inc_flag != true {
                    let mut vis_flag = true;
                    for vb in row+1..self.tree_map.len() {
                        if self.tree_map[vb][col] >= current_val {
                            vis_flag = false;
                        }
                    }
                    if vis_flag == true {
                        inc_flag = true;
                    }
                }
                if inc_flag {
                    visible_trees += 1;
                }
            }
        }
        println!("Part1: Number of visible trees is {visible_trees}");
    }

    fn part2(&self) {
        let mut highest_score = 0;
        for row in 0..self.tree_map.len() {
            for col in 0..self.tree_map[row].len() {
                let current_val = self.tree_map[row][col];
                let mut north_score = 0;
                let mut south_score = 0;
                let mut west_score = 0;
                let mut east_score = 0;

                for hl in (0..col).rev() {
                    west_score += 1;
                    if self.tree_map[row][hl] >= current_val {
                        break;
                    }
                }

                for hr in col+1..self.tree_map[row].len() {
                    east_score += 1;
                    if self.tree_map[row][hr] >= current_val {
                        break;
                    }
                }

                for vt in (0..row).rev() {
                    north_score += 1;
                    if self.tree_map[vt][col] >= current_val {
                        break;
                    }
                }

                for vb in row+1..self.tree_map.len() {
                    south_score += 1;
                    if self.tree_map[vb][col] >= current_val {
                        break;
                    }
                }
                let scenic_score = north_score * south_score * west_score * east_score;
                if scenic_score > highest_score {
                    highest_score = scenic_score;
                }
            }
        }

        println!("Part2: Highest scenic score possible is {highest_score}");
    }
}


fn main() {
    let mut day = Day8::new();
    day.parse();
    day.part1();
    day.part2();
}
