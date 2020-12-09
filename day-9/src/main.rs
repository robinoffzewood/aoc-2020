use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    // part 1
    let xmas_tst_msg = XmasMsg::from_file("example.txt", 5);
    let tst_result = xmas_tst_msg.find_first_invalid_number();
    assert_eq!(tst_result, 127);

    let xmas_msg = XmasMsg::from_file("input.txt", 25);
    let result = xmas_msg.find_first_invalid_number();
    println!("First invalid number = {}", result);

    // part 2
    let tst_weakness = xmas_tst_msg.find_encryption_weakness(tst_result);
    assert_eq!(tst_weakness, 62);
    let weakness = xmas_msg.find_encryption_weakness(result);
    println!("Encryption weakness = {}", weakness);

    // exec time
    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}
struct XmasMsg {
    preamble: usize,
    list: Vec::<usize>
}

impl XmasMsg {
    fn from_file(f_name: &str, preamble: usize) -> XmasMsg {
        let mut list : Vec<usize> = Vec::new();
        let str_in = fs::read_to_string(f_name).expect("Error in reading file");
        for l in str_in.split_whitespace() {
            let val = l.parse::<usize>().unwrap(); // voluntary panic if it's not a number
            list.push(val);
        }
        println!("loaded {} lines of XMAS message, with preamble = {}", list.len(), preamble);
        XmasMsg {preamble, list}
    }

    fn find_first_invalid_number(&self) -> usize {
        // keep a vector of the (n) * (n-1) / 2 acceptable number, n being the preamble value
        let capacity = self.preamble * (self.preamble - 1) / 2;
        for i in self.preamble..self.list.len() - 1 {
            let mut acceptable: Vec<usize> = Vec::with_capacity(capacity);
            for j in (i - self.preamble)..i {
                for k in (j + 1)..i {
                    acceptable.push(self.list[j] + self.list[k]);
                }
            }
            let nb = self.list.get(i).unwrap();
            if acceptable.contains(nb) { // valid
                continue;
            }
            else { // found an invalid one!
                return self.list.get(i).unwrap().clone();
            }
        }
        0
    }

    fn find_encryption_weakness(&self, target: usize) -> usize {
        for i in 0..self.list.len() {
            // init the sum with the first element of the new slice
            let mut contiguous_sum = self.list[i];
            for j in (i+1)..self.list.len() {
                // add up the next element in the current candidate slice
                contiguous_sum += self.list[j] ;
                if contiguous_sum > target {
                    break;
                }
                if contiguous_sum == target { // found
                    let mut set: Vec<usize> = Vec::new();
                    for k in i..(j+1) {
                        set.push(self.list[k]);
                    }
                    set.sort();
                    return set.first().unwrap() + set.last().unwrap();
                }
            }
        }
        0
    }
}