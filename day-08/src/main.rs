use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let str_test = fs::read_to_string("example.txt").expect("Error in reading file");
    let mut test_cpu : Cpu = Cpu{ acc: 0, add: 0, program: Vec::new(), aborted: false};
    test_cpu.load(&str_test);
    test_cpu.run();
    assert_eq!(test_cpu.aborted, true ); // test program must abort!
    assert_eq!(test_cpu.acc, 5);

    let str_in = fs::read_to_string("input.txt").expect("Error in reading file");
    let mut cpu : Cpu = Cpu{ acc: 0, add: 0, program: Vec::new(), aborted: false};
    cpu.load(&str_in);
    cpu.run();
    println!("program aborted = {} ; Acc = {}", cpu.aborted, cpu.acc);

    // reset the cpu
    let mut cpu_2 = Cpu{ acc: 0, add: 0, program: Vec::new(), aborted: false};
    cpu_2.load(&str_in);
    cpu_2.fix();
    println!("program aborted = {} ; Acc = {}", cpu_2.aborted, cpu_2.acc);

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}


#[derive(Clone, Debug)]
enum OpCode {
    Nop (i32),
    Acc (i32),
    Jmp (i32)
}
#[derive(Clone, Debug)]
struct Instruction {
    op: OpCode,
    done: bool
}

struct Cpu {
    acc: i32, // accumulator value
    add: usize, // current Instruction address
    program: Vec<Instruction>,
    aborted: bool
}

impl Cpu {
    fn load (&mut self, input: &str) {
        for l in input.split("\n") {
            let (op_code, val) = l.split_at(3);
            let val = val.trim().replace("+", "");
            let val = match val.parse::<i32>(){ Ok(x) => x, Err(_) => 0};
            match op_code {
                "nop" => self.program.push(Instruction{ op: OpCode::Nop(val), done: false}),
                "acc" => self.program.push(Instruction{ op: OpCode::Acc(val), done: false}),
                "jmp" => self.program.push(Instruction{ op: OpCode::Jmp(val), done: false}),
                _ => continue
            }
        }
        //println!("loaded {} lines of santa code", self.program.len());
    }

    // return true if it finished at the last instruction, false either.
    fn run(&mut self) {
        self.add = 0;
        let mut instr = self.program.get_mut(self.add).unwrap();
        while instr.done == false {
            match instr.op {
                OpCode::Nop(_x) => self.add += 1,
                OpCode::Acc(x) => { self.acc += x ; self.add += 1},
                OpCode::Jmp(x) => self.add = ((self.add as i32) + x) as usize,
            }
            instr.done = true;

            if self.add >= self.program.len() { // is it end of program?
                self.aborted = false;
                return;
            }
            instr = self.program.get_mut(self.add).unwrap();
        }
        self.aborted = true;
    }

    fn fix(&mut self) {
        // Make modification of the original program (nop by jmp), and try until run() returns
        let mut cpu_2: Cpu = Cpu { acc: 0, add: 0, program: self.program.clone(), aborted: false };
        for nth in 0..self.program.len() {
            cpu_2 = Cpu { acc: 0, add: 0, program: self.program.clone(), aborted: false };
            if let Some(mut modified_instr) = cpu_2.program.get_mut(nth)
            {
                match modified_instr.op
                {
                    OpCode::Nop(x) => {
                        modified_instr.op = OpCode::Jmp(x);
                        cpu_2.run();
                        if !cpu_2.aborted { // program is fixed!
                            break;
                        }
                    },
                    OpCode::Jmp(x) => {
                        modified_instr.op = OpCode::Nop(x);
                        cpu_2.run();
                        if !cpu_2.aborted { // program is fixed!
                            break;
                        }
                    },
                    OpCode::Acc(_x) => { continue; }
                }
            }
        }
        self.acc = cpu_2.acc;
        self.add = cpu_2.add;
        self.aborted = cpu_2.aborted;
        self.program = cpu_2.program;
    }
}