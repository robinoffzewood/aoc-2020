use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let start = Instant::now();

    let mut tst_prog = Program::from_file("example.txt");
    let mut prog = Program::from_file("input.txt");

    // part 1
    tst_prog.run();
    assert_eq!(165, tst_prog.mem_sum());

    prog.run();
    println!("sum of memory = {}", prog.mem_sum());

    // part2
    let mut tst_prog2 = Program::from_file("example2.txt");
    tst_prog2.floating_addrs = true;
    tst_prog2.run();
    assert_eq!(208, tst_prog2.mem_sum());

    prog.mem.clear();
    prog.floating_addrs = true;
    prog.run();
    println!("sum of memory (floating address) = {}", prog.mem_sum());

    // exec time
    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}
enum OpCode{
    Mask,
    Write
}
struct Instruction {
    op : OpCode,
    ad: usize,
    val: usize,
    mask: String
}
struct Program {
    mem: HashMap<usize, usize>,
    or_mask: usize,
    and_mask: usize,
    addr_mask: String,
    instructions: Vec<Instruction>,
    floating_addrs: bool,
}

impl Program {
    fn run(&mut self) {
        // execute all the instructions, updating the memory
        for instr in &self.instructions {
            match instr.op {
                OpCode::Mask => {
                    // In part 1, the mask is computed once for all the upcoming Write instructions.
                    // In part 2, with the floating address, we'll need to keep the mask value as a string,
                    //  to apply it to each address we want to write to
                    if self.floating_addrs {
                        self.addr_mask = instr.mask.clone();
                    }
                    else {
                        let masks= Program::make_mask(&instr.mask);
                        self.or_mask = masks.0;
                        self.and_mask = masks.1;
                    }
                },
                OpCode::Write => {
                    let mut addr_to_write: Vec<usize> = Vec::new();
                    let val;
                    if self.floating_addrs {
                        addr_to_write = self.get_addrs_list(instr.ad);
                        val = instr.val;
                    }
                    else {
                        addr_to_write.push(instr.ad);
                        val = self.apply_mask(instr.val);
                    }
                    for addr in addr_to_write {
                        self.mem.insert(addr.clone(), val); // if memory cell already exist, it will be updated
                    }
                }
            }
        }
    }

    fn get_addrs_list(&self, ad : usize) -> Vec<usize> {
        // For each AND mask in the list
        //  For each address already in the address list, apply the AND mask and push it to the address list
        let and_masks_list = Program::make_addrs_masks(&self.addr_mask);

        // initialize the first address with the address in the instruction ORed with the OR mask obtained in replacing 'X' by '1'
        let or_mask = self.addr_mask.replace("X", "1");
        let or_mask = usize::from_str_radix(&or_mask, 2).unwrap();
        let mut addrs_list = vec![ad | or_mask];

        for mask in and_masks_list {
            let mut new_addrs_list = Vec::new();
            for addr in &addrs_list {
                new_addrs_list.push(addr & mask);
            }
            addrs_list.append(&mut new_addrs_list);
        }
        addrs_list
    }

    fn make_addrs_masks(str_in: &str) -> Vec<usize> {
        // For each 'X' in the mask, create an AND mask with a '0' instead of the 'X'
        //  and '1's everywhere else
        let mut addrs_list = Vec::new();
        let mut next_mask = str_in.to_string();
        loop {
            if !next_mask.contains("X") {
                break;
            }
            let new_mask = next_mask.replace("0", "1");
            let new_mask = new_mask.replacen("X", "0", 1);
            let new_mask = new_mask.replace("X", "1");
            addrs_list.push(usize::from_str_radix(&new_mask, 2).unwrap());
            next_mask = next_mask.replacen("X", "1", 1);
        }
        addrs_list
    }

    // Return (or, and) masks
    fn make_mask(val:&str) -> (usize, usize) {
        // OR mask is made by replacing X with 0, and keeping 1 as they are
        // AND mask is made by replacing X with 1, and keeping 0 as they are
        let or_str = val.replace("X", "0");
        let and_str = val.replace("X", "1");
        let or_mask = usize::from_str_radix(&or_str, 2).unwrap();
        let and_mask = usize::from_str_radix(&and_str, 2).unwrap();
        (or_mask, and_mask)
    }

    fn apply_mask(&self, val:usize) -> usize {
        let result = val | self.or_mask;
        let result = result & self.and_mask;
        result
    }

    fn mem_sum(&self) -> usize {
        // sum up all the values in the memory
        let mut sum : usize = 0;
        for val in self.mem.values() {
            sum += val.clone();
        }
        sum
    }

    fn from_file(f_name: &str) -> Program {
        let str_in = fs::read_to_string(f_name).expect("Error in reading file");
        let mut instructions = Vec::new();
        for line in str_in.lines() {
            match &line[0..4] {
                "mask" => {
                    let mut ite = line.split(" = ");
                    ite.next();
                    instructions.push(Instruction{
                        op: OpCode::Mask,
                        ad: 0,
                        val: 0,
                        mask: ite.next().unwrap().to_string()});
                },
                "mem[" => {
                    let ad = line[4..].split("]").next().unwrap().parse::<usize>().unwrap();
                    let mut ite = line.split(" = ");
                    ite.next();
                    let val = ite.next().unwrap().parse::<usize>().unwrap();
                    instructions.push(Instruction{
                        op: OpCode::Write,
                        ad,
                        val,
                        mask: "".to_string()
                    });
                },
                _ => break
            }
        }
        Program {
            mem: HashMap::new(),
            or_mask: 0,
            and_mask: usize::max_value(),
            addr_mask: "".to_string(),
            instructions,
            floating_addrs: false
        }
    }
}