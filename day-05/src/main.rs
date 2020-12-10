use std::time::Instant;
use std::fs;
use std::cmp::max;

fn main() {
    let start = Instant::now();
    test();

    let contents = fs::read_to_string("input.txt").expect("Error in reading file");
    let mut seat_id_max = 0;
    let mut seat_ids:Vec<u32> = Vec::new();

    for boarding_pass in contents.lines() {
        let (r, c) = to_bdg_pass(boarding_pass);
        let seat_id = r * 8  +c;
        seat_id_max = max(seat_id_max, seat_id);
        seat_ids.push(seat_id);
    }
    seat_ids.sort();
    let mut id_prev = *seat_ids.first().unwrap();

    for id in seat_ids {
        if id > id_prev + 1 {
            // found it! it's id_prev + 1
            break;
        }
        id_prev = id;
    }
    println!("seat_id_max = {}", seat_id_max);
    println!("Santa seat id = {}", id_prev + 1);
    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn test() {
    let (r, c) = to_bdg_pass("BFFFBBFRRR");
    assert_eq!(r, 70);
    assert_eq!(c, 7);
    let (r, c) = to_bdg_pass("FFFBBBFRRR");
    assert_eq!(r, 14);
    assert_eq!(c, 7);
    let (r, c) = to_bdg_pass("BBFFBBFRLL");
    assert_eq!(r, 102);
    assert_eq!(c, 4);
}

fn to_bdg_pass(s: &str) -> (u32, u32) {
    let row = &s[0..7].replace("F", "0").replace("B","1");
    let col = &s[7..10].replace("L", "0").replace("R","1");
    let row = to_int(row);
    let col = to_int(col);
    (row, col)
}

fn to_int (b: &str) -> u32 {
    let mut result = 0;
    for c in b.chars() {
        result *= 2;
        if let Some(digit) = c.to_digit(2) {
            result += digit;
        }
    }
    result
}