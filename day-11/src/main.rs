use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut tst_seats = Seats::from_file("example.txt");
    let mut seats = Seats::from_file("input.txt");

    // part 1
    tst_seats.occupy(1);
    assert_eq!(tst_seats.occupied, 37);
    seats.occupy(1);
    println!("Nb of occupied seats = {}", seats.occupied);

    // part 2
    let mut tst_seats = Seats::from_file("example.txt");
    let mut seats = Seats::from_file("input.txt");
    tst_seats.occupy(2);
    assert_eq!(tst_seats.occupied, 26);
    seats.occupy(2);
    println!("Nb of occupied seats = {}", seats.occupied);

    // exec time
    let duration = start.elapsed();
    println!("Finished after {:?}", duration);

}

enum SeatState {
    Floor, // .
    Empty, // L
    Occup // #
}

struct Xy {
    x: usize,
    y: usize
}

struct Seats {
    seats: Vec<SeatState>,
    room_dim: Xy,
    occupied: i32
}

impl Seats {
    fn occupy(&mut self, part : i32) {
        let mut changes = 1;
        while changes != 0{
            changes = self.run(part);
        }
    }

    fn run(&mut self, part: i32) -> i32 {
        let mut changes = 0;
        let mut seats_after : Vec<SeatState> = Vec::with_capacity(self.seats.len());
        self.occupied = 0; // reset!
        for y in 0..self.room_dim.y {
            for x in 0..self.room_dim.x {
                let mut occupied=0;
                if part == 1 {occupied = self.compute_occupied(x, y, 1);}
                if part == 2 {occupied = self.compute_occupied(x, y, i32::max_value());}
                let state = self.get_state(x, y);
                //println!("From ({}, {})  -> occup = {}", x, y, occupied);
                let new_state;
                match &state {
                    SeatState::Empty => {
                        if occupied == 0 {new_state = SeatState::Occup; changes += 1; self.occupied += 1; }
                        else {new_state = state;}
                    },
                    SeatState::Occup => {
                        let mut threshold = 4;
                        if part == 2 {threshold = 5;}
                        if occupied >= threshold {new_state = SeatState::Empty; changes += 1}
                        else {new_state = state;  self.occupied += 1;}
                    },
                    SeatState::Floor => new_state = state,
                }
                seats_after.push(new_state);
            }
        }
        self.seats = seats_after;
        changes
    }

    fn get_state(&self, x: usize, y: usize) -> SeatState {
        let idx = y * self.room_dim.x + x;
        return match self.seats[idx] {
            SeatState::Empty => SeatState::Empty,
            SeatState::Occup => SeatState::Occup,
            SeatState::Floor => SeatState::Floor
        }
    }

    fn compute_occupied(&self, x: usize, y: usize, iter_max: i32) -> i32 {
        let mut cnt = 0;
        let directions = self.get_exploring_steps(x, y);
        //println!("{:?}", directions);
        for dir in directions {
            match self.get_state_in_dir(x, y, dir, iter_max) {
                SeatState::Occup => cnt += 1,
                _ => continue,
            }
        }
        cnt
    }

    fn get_exploring_steps(&self, x: usize, y: usize) -> Vec<(i32, i32)> {
        // return the 8 steps from (-1, -1) to (1, 1) excluding the moves that would get out of bounds, and excluding (0, 0)
        let x_lo = if x == 0 {0} else {-1};
        let x_hi = if x == self.room_dim.x - 1 {0} else {1};
        let y_lo = if y == 0 {0} else {-1};
        let y_hi = if y == self.room_dim.y - 1 {0} else {1};
        let mut steps : Vec<(i32, i32)> = Vec::new();
        for y_ in y_lo..y_hi+1 {
            for x_ in x_lo..x_hi+1 {
                if y_ == 0 && x_ == 0 {continue;}
                steps.push((x_, y_));
            }
        }
        steps
    }


    fn get_state_in_dir(&self, x: usize, y: usize, dir: (i32, i32), iter_max: i32) -> SeatState {
        let mut cur_pos = (x, y);
        let mut cnt = 0;
        loop {
            // stop if we reached the max number of iteration
            if cnt >= iter_max {break;}
            cnt += 1;
            // stop if we reached the latest seat
            let next_pos :(i32, i32) = (cur_pos.0 as i32 + dir.0, cur_pos.1 as i32 + dir.1);
            if next_pos.0 < 0 || next_pos.0 >= self.room_dim.x as i32 {break;}
            if next_pos.1 < 0 || next_pos.1 >= self.room_dim.y as i32 {break;}

            cur_pos.0 = next_pos.0 as usize;
            cur_pos.1 = next_pos.1 as usize;
            //println!("({}, {})", cur_pos.0, cur_pos.1);
            let idx = cur_pos.1 * self.room_dim.x + cur_pos.0;
            match self.seats[idx] {
                SeatState::Empty => return SeatState::Empty,
                SeatState::Occup => return SeatState::Occup,
                SeatState::Floor => continue,
            }
        }
        SeatState::Floor
    }

    fn from_file(f_name:&str) -> Seats {
        let str_in = fs::read_to_string(f_name).expect("Error in reading file");

        let h = str_in.split_whitespace().count();
        let w= str_in.split_whitespace().nth(0).unwrap().len();
        let room_dim = Xy {x: w, y: h};
        let mut seats: Vec<SeatState> = Vec::new();

        for l in str_in.split_whitespace()  {
            for c in l.chars() {
                match c {
                    '.' => seats.push(SeatState::Floor),
                    'L' => seats.push(SeatState::Empty),
                    _ => seats.push(SeatState::Occup)
                }
            }
        }
        //println!("Loaded {} x {} = {} seats", room_dim.x, room_dim.y, seats.len());
        Seats { seats, room_dim, occupied : 0 }
    }
}