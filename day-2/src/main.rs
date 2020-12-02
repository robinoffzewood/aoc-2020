use std::env;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct PwdPolicy {
    letter: String,
    min: usize,
    max: usize,
}

#[derive(Debug)]
struct PwdValidator {
    policy: PwdPolicy,
    password: String
}

impl PwdValidator {
    fn create(policy: String, password: String) -> PwdValidator {
        // policy is of like "2-8 q"
        let minmax_char: Vec<&str> = policy.split_whitespace().collect();
        let minmax: Vec<usize> = minmax_char[0].split("-").map(|i| i.parse().expect("parse error")).collect();
        let ch = minmax_char[1];
        let policy = PwdPolicy {
            letter: ch.to_string(),
            min: minmax[0],
            max: minmax[1]
        };
        // return the object created below
        PwdValidator {
            policy,
            password
        }
    }
    fn is_valid(&self) -> bool {
        let cnt = self.password.matches(&self.policy.letter).count();
        return if cnt < self.policy.min || cnt > self.policy.max {
            false
        } else {
            true
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let start = Instant::now();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut valid_cnt = 0;
    for line in contents.lines() {
        let policy_pwd: Vec<&str> = line.split(":").collect();
        let password = PwdValidator::create(policy_pwd[0].to_string(), policy_pwd[1].trim_start().to_string());
        //println!{"{:?}", password};
        if password.is_valid() == true {
            valid_cnt += 1;
        }
    }
    println!("Invalid password = {}", valid_cnt);

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}
