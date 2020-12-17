use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut tst_shuttles = Shuttles::from_file("example.txt");
    let mut shuttles = Shuttles::from_file("input.txt");

    // part 1
    let (bus_id, ttw) = tst_shuttles.find_earliest();
    println!("bus_id = {} ; Time to wait = {} => {}", bus_id, ttw, bus_id * ttw);
    assert_eq!(295, bus_id * ttw);
    let (bus_id, ttw) = shuttles.find_earliest();
    println!("bus_id = {} ; Time to wait = {} => {}", bus_id, ttw, bus_id * ttw);

    // exec time
    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

struct Shuttles {
    buses: Vec<usize>,
    departure_time: usize
}

impl Shuttles {
    // return (bus id, time to wait)
    fn find_earliest(&self) -> (usize, usize) {
        let mut earliest : (usize, usize) = (0, usize::max_value());
        for id in &self.buses {
            // +------+-------+ <- bus id = interval_length
            //             ^    <- departure_time
            //        <--->     <- departure_time % id = miss_delay
            //              <-> <- interval_length - miss_delay = time_to_wait
            let time_to_wait = id - (self.departure_time % id);
            if time_to_wait < earliest.1 {
                earliest = (id.clone(), time_to_wait);
            }
        }
        earliest
    }

    fn from_file(f_name: &str) -> Shuttles {
        let str_in = fs::read_to_string(f_name).expect("Error in reading file");
        let mut buses: Vec<usize> = Vec::new();
        let first_line = str_in.split_whitespace().nth(0).unwrap();
        let depart_time = first_line.parse::<usize>().unwrap();
        let second_line = str_in.split_whitespace().nth(1).unwrap();
        for id in second_line.split(",") {
            if id != "x" {
                let bus_id = id.parse::<usize>().unwrap();
                buses.push(bus_id)
            }
        }
        println!("Loaded {} buses", buses.len());
        Shuttles { buses, departure_time: depart_time }
    }
}