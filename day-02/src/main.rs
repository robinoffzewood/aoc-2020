use std::env;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
enum PwdPolicyKind {
    Old,
    New
}

#[derive(Debug)]
struct PwdPolicy {
    kind: PwdPolicyKind,
    letter: String,
    left: usize,
    right: usize,
}

#[derive(Debug)]
struct PwdValidator {
    policy: PwdPolicy,
    password: String
}

impl PwdValidator {
    fn create_policy(kind: PwdPolicyKind, policy: String, password: String) -> PwdValidator {
        // policy is of like "2-8 q"
        let left_right_char: Vec<&str> = policy.split_whitespace().collect();
        let left_right: Vec<usize> = left_right_char[0].split("-").map(|i| i.parse().expect("parse error")).collect();
        let ch = left_right_char[1];
        let policy = PwdPolicy {
            kind,
            letter: ch.to_string(),
            left: left_right[0],
            right: left_right[1]
        };
        // return the object created below
        PwdValidator {
            policy,
            password
        }
    }
    fn is_valid(&self) -> bool {
        match self.policy.kind {
            PwdPolicyKind::Old => self._is_valid_old_pol(),
            PwdPolicyKind::New => self._is_valid_new_pol()
        }
    }

    fn _is_valid_old_pol(&self) -> bool {
        let cnt = self.password.matches(&self.policy.letter).count();
        return if cnt < self.policy.left || cnt > self.policy.right {
            false
        } else {
            true
        }
    }

    fn _is_valid_new_pol(&self) -> bool {
        let l_char = self.password.chars().nth(self.policy.left - 1).unwrap();
        let r_char = self.password.chars().nth(self.policy.right - 1).unwrap();

        let is_left_ok : bool = l_char.to_string() == self.policy.letter;
        let is_right_ok : bool = r_char.to_string() == self.policy.letter;
        is_left_ok ^ is_right_ok // ^ is a XOR operator
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let start = Instant::now();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut old_pol_valid_cnt = 0;
    let mut new_pol_valid_cnt = 0;
    for line in contents.lines() {
        let policy_pwd: Vec<&str> = line.split(":").collect();
        let password_old_pol = PwdValidator::create_policy(PwdPolicyKind::Old, policy_pwd[0].to_string(), policy_pwd[1].trim_start().to_string());
        let password_new_pol = PwdValidator::create_policy(PwdPolicyKind::New, policy_pwd[0].to_string(), policy_pwd[1].trim_start().to_string());
        //println!{"{:?}", password};
        if password_old_pol.is_valid() {
            old_pol_valid_cnt += 1;
        }
        if password_new_pol.is_valid() {
            new_pol_valid_cnt += 1;
        }
    }
    println!("Valid password according to *OLD* policy = {}", old_pol_valid_cnt);
    println!("Valid password according to *NEW* policy = {}", new_pol_valid_cnt);

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}
