use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let start = Instant::now();

    assert_eq!(0, nth_number("0,3,6", 10));
    assert_eq!(436, nth_number("0,3,6", 2020));
    assert_eq!(1, nth_number("1,3,2", 2020));
    assert_eq!(10, nth_number("2,1,3", 2020));
    assert_eq!(27, nth_number("1,2,3", 2020));
    assert_eq!(78, nth_number("2,3,1", 2020));
    assert_eq!(438, nth_number("3,2,1", 2020));
    assert_eq!(1836, nth_number("3,1,2", 2020));

    let result = nth_number("13,16,0,12,15,1", 2020);
    println!("The 2020th number spoken will be {}", result);

    let result = nth_number("13,16,0,12,15,1", 300_000_00);
    println!("The 300_000_000 number spoken will be {}", result);

    // exec time
    let duration = start.elapsed();
    println!("Finished after {:?}", duration);

}

fn nth_number(str_in: &str, last_turn: usize) -> usize {
    let mut list_spoken : HashMap<usize, usize> = HashMap::new();
    let mut turn : usize = 0;
    let mut speaking: usize = 0;
    let mut already_spoken= None;

    // init
    let starting_numbers :Vec<&str> = str_in.split(",").collect();
    for starting_nb in starting_numbers {
        turn += 1;
        speaking = starting_nb.parse::<usize>().unwrap();
        already_spoken = list_spoken.insert(speaking, turn);
    }

    while turn < last_turn {
        turn += 1;
        match already_spoken {
            None => speaking = 0,
            Some(last_turn_it_was_spoken) => speaking = turn - 1 - last_turn_it_was_spoken,
        }
        already_spoken = list_spoken.insert(speaking, turn);
        //println!("turn = {} speaking = {}", turn, speaking);
    }
    speaking
}