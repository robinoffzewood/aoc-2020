use std::{env, fs};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut f_in = "example.txt".to_string();
    if let Some(arg_1) = env::args().nth(1) { // learning how to use `if let` today
        f_in = arg_1;
    }
    let contents = fs::read_to_string(f_in).expect("Error in reading file");

    let mut passports= Vec::new();
    let mut str_in= "".to_string();

    // Fill passports list from input
    for line in contents.lines() {
        // while no empty line, concatenate the string
        if line.len() > 0 {
            str_in = str_in + " " + line;
        }
        else { // create a Passport object from the string, and push it.
            passports.push(Passport::from_str(&str_in));
            str_in = "".to_string(); // reset the string
        }
    }
    // ...don't forget the last one
    passports.push(Passport::from_str(&str_in));

    // Count the nb of valid passports
    let mut valid_1 = 0;
    let mut valid_2 = 0;
    for p in passports {
        if p.is_valid_1() {
            valid_1 += 1;
        }
        if p.is_valid_2() {
            valid_2 += 1;
        }
    }
    let duration = start.elapsed();
    println!("Finished after {:?}", duration);

    println!("Valid passport with rule 1 = {}", valid_1);
    println!("Valid passport with rule 2 = {}", valid_2);
}


#[derive(Debug, Default)]
struct Passport {
    byr:String, //(Birth Year)
    iyr:String, //(Issue Year)
    eyr:String, //(Expiration Year)
    hgt:String, //(Height)
    hcl:String, //(Hair Color)
    ecl:String, //(Eye Color)
    pid:String, //(Passport ID)
    cid:String, //(Country ID)
}

impl Passport {
    fn is_valid_1(&self) -> bool {
        return self.byr.len() > 0 &&
            self.iyr.len() > 0 &&
            self.eyr.len() > 0 &&
            self.hgt.len() > 0 &&
            self.hcl.len() > 0 &&
            self.ecl.len() > 0 &&
            self.pid.len() > 0 &&
            true //self.cid.len() > 0 // ignore cid
    }

    fn is_valid_2(&self) -> bool {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        return self.byr.len() == 4 && self.byr.parse::<i32>().unwrap() >= 1920 && self.byr.parse::<i32>().unwrap() <= 2002 &&
            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            self.iyr.len() == 4 && self.iyr.parse::<i32>().unwrap() >= 2010 && self.iyr.parse::<i32>().unwrap() <= 2020 &&
            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            self.eyr.len() == 4 && self.eyr.parse::<i32>().unwrap() >= 2020 && self.eyr.parse::<i32>().unwrap() <= 2030 &&
            self.is_hgt_valid() &&
            self.is_hcl_valid() &&
            self.is_ecl_valid() &&
            self.is_pid_valid() &&
            // cid (Country ID) - ignored, missing or not.
            true
    }

    fn is_hgt_valid(&self) -> bool {
        // hgt (Height) - a number followed by either cm or in:
        //   If cm, the number must be at least 150 and at most 193.
        //   If in, the number must be at least 59 and at most 76.
        let hgt = &self.hgt;
        if hgt.len() > 0 {
            if hgt.ends_with("cm") {
                let h = hgt.strip_suffix("cm").unwrap().parse::<i32>().unwrap();
                if h >= 150 && h <= 193 {
                    return true;
                }
            }
            if hgt.ends_with("in") {
                let h = hgt.strip_suffix("in").unwrap().parse::<i32>().unwrap();
                if h >= 59 && h <= 76 {
                    return true;
                }
            }
        }
        return false;
    }

    fn is_hcl_valid(&self) -> bool {
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        let hcl = &self.hcl;
        if hcl.len() == 7 {
            if hcl.starts_with("#"){
                if hcl[1..6].chars().all(char::is_alphanumeric) {
                    return true;
                }
            }
        }
        return false;
    }

    fn is_ecl_valid(&self) -> bool {
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        let ecl = &self.ecl;
        if ecl.len() > 0 {
            let r : bool;
            match ecl.as_str() {
                "amb" => r=true,
                "blu" => r=true,
                "brn" => r=true,
                "gry" => r=true,
                "grn" => r=true,
                "hzl" => r=true,
                "oth" => r=true,
                _ => r=false
            }
            return r;
        }
        return false;
    }

    fn is_pid_valid(&self) -> bool {
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        let pid = &self.pid;
        let r = pid.len() == 9 && pid.chars().all(char::is_numeric);
        return r;
    }

    fn from_str(str_in: &String) -> Passport {
        let mut p:Passport = Passport{..Default::default()};
        let fields = str_in.split_whitespace();
        for f in fields {
            let kv :Vec<&str> = f.split(":").collect();
            match kv[0] {
                "byr" => p.byr = kv[1].to_string(),
                "iyr" => p.iyr = kv[1].to_string(),
                "eyr" => p.eyr = kv[1].to_string(),
                "hgt" => p.hgt = kv[1].to_string(),
                "hcl" => p.hcl = kv[1].to_string(),
                "ecl" => p.ecl = kv[1].to_string(),
                "pid" => p.pid = kv[1].to_string(),
                "cid" => p.cid = kv[1].to_string(),
                _ => continue
            }
        }
        p
    }
}