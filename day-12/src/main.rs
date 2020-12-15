use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut tst_ship = Ship::from_file("example.txt");
    let mut ship = Ship::from_file("input.txt");

    // part 1
    tst_ship.follow_instructions(1);
    assert_eq!(tst_ship.manhattan_distance(), 25);
    ship.follow_instructions(1);
    println!("Manhattan distance (part 1) = {}", ship.manhattan_distance());

    // part 2
    tst_ship.reset();
    tst_ship.follow_instructions(2);
    assert_eq!(tst_ship.manhattan_distance(), 286);
    ship.reset();
    ship.follow_instructions(2);
    println!("Manhattan distance (part 2) = {}", ship.manhattan_distance());

    // exec time
    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

struct Ship {
    position : (i32, i32),
    direction: (i32, i32),
    way_point: (i32, i32),
    instructions: Vec<Instr>
}
enum Action {North, East, South, West, Left, Right, Forward}
struct Instr {
    action: Action,
    val: i32,
}
impl Ship {
    fn follow_instructions(&mut self, part: i32) {
        for instr in &self.instructions {
            match instr.action {
                Action::North => {
                    if part == 2 { self.way_point.1 += instr.val; }
                    else { self.position.1 += instr.val; }
                },
                Action::East  => {
                    if part == 2 { self.way_point.0 += instr.val; }
                    else { self.position.0 += instr.val; }
                },
                Action::South => {
                    if part == 2 { self.way_point.1 -= instr.val; }
                    else {self.position.1 -= instr.val;}
                },
                Action::West  => {
                    if part == 2 { self.way_point.0 -= instr.val; }
                    else { self.position.0 -= instr.val;}
                },
                Action::Left => {
                    if part == 2 { self.way_point = Ship::turn(self.way_point, instr.val);}
                    else {self.direction = Ship::turn(self.direction, instr.val);}
                },
                Action::Right => {
                    if part == 2 {
                        self.way_point = Ship::turn(self.way_point, -instr.val);
                    }
                    else {self.direction = Ship::turn(self.direction, -instr.val); }
                },
                Action::Forward => {
                    if part == 2 {
                        self.position.0 += self.way_point.0 * instr.val;
                        self.position.1 += self.way_point.1 * instr.val;
                    }
                    else {
                        self.position.0 += self.direction.0 * instr.val;
                        self.position.1 += self.direction.1 * instr.val;
                    }
                }
            }
        }
    }

    fn turn(direction: (i32, i32), angle: i32) -> (i32, i32) {
        let cos_theta = f32::cos(angle as f32 / 180.0 * std::f32::consts::PI);
        let sin_theta = f32::sin(angle as f32 / 180.0 * std::f32::consts::PI);
        let x = direction.0 as f32;
        let y = direction.1 as f32;
        let new_x = x * cos_theta - y * sin_theta;
        let new_y = x * sin_theta + y * cos_theta;
        (new_x.round() as i32, new_y.round() as i32)
    }

    fn manhattan_distance(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }

    fn reset(&mut self)
    {
        self.direction = (1, 0);
        self.position = (0, 0);
        self.way_point = (10, 1);
    }

    fn from_file(f_name: &str) -> Ship {
        let str_in = fs::read_to_string(f_name).expect("Error in reading file");
        let mut instructions: Vec<Instr> = Vec::new();

        for l in str_in.split_whitespace() {
            let char_action = &l[0..1];
            let value = l[1..].parse::<i32>().unwrap();
            match char_action {
                "N" => instructions.push(Instr { action: Action::North, val: value }),
                "E" => instructions.push(Instr { action: Action::East, val: value }),
                "S" => instructions.push(Instr { action: Action::South, val: value }),
                "W" => instructions.push(Instr { action: Action::West, val: value }),
                "L" => instructions.push(Instr { action: Action::Left, val: value }),
                "R" => instructions.push(Instr { action: Action::Right, val: value }),
                "F" => instructions.push(Instr { action: Action::Forward, val: value }),
                _ => break
            }
        }
        println!("Loaded {} instructions", instructions.len());
        Ship { position: (0, 0), direction: (1, 0), way_point: (10, 1), instructions }
    }
}