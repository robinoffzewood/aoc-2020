use std::env;
use std::fs;

const SLOPE_X: i32 = 3;
const SLOPE_Y: i32 = 1;

struct Xy {
    x: i32,
    y: i32
}

struct Slope {
    grid_flatten: Vec<bool>,
    grid_dim: Xy,
    santa: Xy,
    delta: Xy,
    trees_hit:u16
}

impl Slope {
    fn from_lines (lines: &String) -> Slope {
        let h = lines.split_whitespace().count();
        let w= lines.split_whitespace().nth(0).unwrap().len();
        let grid_dim = Xy {x: w as i32, y: h as i32};
        let mut grid_flatten: Vec<bool> = Vec::new();

        for l in lines.split_whitespace()  {
            for c in l.chars() {
                match c {
                    '#' => grid_flatten.push(true),
                    '.' => grid_flatten.push(false),
                    _ => grid_flatten.push(false)
                }
            }
        }
        Slope {
            grid_flatten,
            grid_dim,
            santa: Xy {x: 0, y: 0},
            delta: Xy {x: SLOPE_X, y: SLOPE_Y },
            trees_hit: 0
        }
    }

    fn here_is_a_tree(&self, here: &Xy) -> bool {
        let idx = here.y * self.grid_dim.x + here.x;
        return self.grid_flatten[idx as usize] == true;
    }

    fn descend (&mut self) {
        while self.santa.y + self.delta.y < self.grid_dim.y {
            self.santa.x = (self.santa.x  + self.delta.x) % self.grid_dim.x;
            self.santa.y += self.delta.y;
            if self.here_is_a_tree(&self.santa) {
                self.trees_hit += 1;
            }
        }
    }

}

fn main() {
    let mut f_in = "example.txt".to_string();
    if let Some(arg_1) = env::args().nth(1) { // learning how to use `if let` today
        f_in = arg_1;
    }
    let contents = fs::read_to_string(f_in).expect("Error in reading file");
    let mut tobogan = Slope::from_lines(&contents);
    tobogan.descend();
    println!{"Outch! hit {} trees!", tobogan.trees_hit};
}
