use std::fs;
use std::time::Instant;
use std::collections::BTreeSet;

fn main() {
    let start = Instant::now();

    let tst_list = Adapters::from_file("example.txt");
    let list = Adapters::from_file("input.txt");

    // part 1
    let tst_result = tst_list.nb_of_1jolt_by_3jolts();
    assert_eq!(tst_result, 22*10);
    let result = list.nb_of_1jolt_by_3jolts();
    println!("The number of 1-jolt differences multiplied by the number of 3-jolt differences = {}", result);

    // part 2
    let tst2_result = tst_list.nb_of_possible_arrangements();
    assert_eq!(tst2_result, 19208);
    let result2 = list.nb_of_possible_arrangements();
    println!("The total number of distinct ways I can arrange the adapters to connect = {}", result2);

    // exec time
    let duration = start.elapsed();
    println!("Finished after {:?}", duration);

}
struct Adapters {
    list: BTreeSet<usize>
}

impl Adapters {
    fn from_file(f_name: &str) -> Adapters {
        let mut list: BTreeSet<usize> = BTreeSet::new();
        let str_in = fs::read_to_string(f_name).expect("Error in reading file");
        for l in str_in.split_whitespace() {
            let val = l.parse::<usize>().unwrap(); // voluntary panic if it's not a number
            list.insert(val);
        }
        //println!("Got {} adapters in my bag", list.len());
        Adapters { list }
    }

    fn nb_of_1jolt_by_3jolts(&self) -> usize {
        // Accumulate the nb of 1 jolt & 3 jolts diff
        let mut one_jolt : usize = 1;  // start at 1, charging outlet matters
        let mut three_jolt : usize = 1;  // start at 1, charging outlet matters
        // First step
        let mut iter = self.list.iter();
        let mut start = iter.next().unwrap();
        for next in iter {
            if next - start == 1 {
                one_jolt += 1;
            } else if next - start == 3 {
                three_jolt += 1;
            }
            start = next;
        }
        one_jolt * three_jolt
    }

    fn nb_of_possible_arrangements(&self) -> usize {
        let max_idx =  self.list.iter().max().unwrap();
        let mut arrangements: Vec<usize> = vec![0; max_idx + 1];
        arrangements[0] = 1; // charging outlet
        for i in &(self.list) {
            let arr_idx = *i;
            if arr_idx >= 3 {
                arrangements[arr_idx] += arrangements[arr_idx - 3];
            }
            if arr_idx >= 2 {
                arrangements[arr_idx] += arrangements[arr_idx - 2];
            }
            arrangements[arr_idx] += arrangements[arr_idx - 1];
        }
        arrangements.last().unwrap().clone()
    }
}

