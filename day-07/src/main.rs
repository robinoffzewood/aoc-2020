use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let str_test = fs::read_to_string("example.txt").expect("Error in reading file");
    let mut test_rules : Rules = Rules {the_rules: HashMap::new(), contained_in: HashMap::new()};
    test_rules.parse_all_bags(&str_test);
    test_rules.compute_contained_in();
    let list = test_rules.list_of_bags_that_can_contain_at_least_one("shiny gold");
    assert_eq!(list.len(), 4);

    let str_in = fs::read_to_string("input.txt").expect("Error in reading file");
    let mut rules : Rules = Rules {the_rules: HashMap::new(), contained_in: HashMap::new()};
    rules.parse_all_bags(&str_in);
    rules.compute_contained_in();
    let list = rules.list_of_bags_that_can_contain_at_least_one("shiny gold");
    println!("There are {} bags that can eventually contain a `shiny gold` bag", list.len());

    let str_test2 = fs::read_to_string("example2.txt").expect("Error in reading file");
    let mut test2_rules : Rules = Rules {the_rules: HashMap::new(), contained_in: HashMap::new()};
    test2_rules.parse_all_bags(&str_test2);
    test2_rules.compute_contained_in();
    let cnt_test2 = test2_rules.nb_of_bags_contained_in("shiny gold") - 1;
    assert_eq!(cnt_test2, 126);

    let cnt = rules.nb_of_bags_contained_in("shiny gold") - 1;
    println!("There are {} bags contained a `shiny gold` bag", cnt);

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}


struct Rules {
    the_rules : HashMap<String<>, Vec<CanContain>>,
    contained_in : HashMap<String<>, Vec<String>>
}

#[derive(Debug, Default)]
struct CanContain {
    cnt: usize,
    name: String
}

impl Rules {
    fn parse_all_bags(&mut self, lines: &String) {
        self.the_rules = HashMap::new();
        for l in lines.split("\n") {
            let (bag_name, bag_props) = self.parse_bag_properties(l);
            self.the_rules.insert(bag_name, bag_props);
        }
    }

    fn list_of_bags_that_can_contain_at_least_one(&self, bag_name: &str) -> Vec<String> {
        let mut list : Vec<String> = Vec::new();
        if let Some(parent_bags) = self.contained_in.get(bag_name) {
            let mut parents_cloned = parent_bags.clone();
            list.append(&mut parents_cloned);
            for parent in parent_bags {
                list.append(&mut self.list_of_bags_that_can_contain_at_least_one(parent).clone());
            }
        }
        list.sort(); // in order to remove duplicates that are not consecutive
        list.dedup(); // deduplicate !
        list
    }

    fn nb_of_bags_contained_in(&self, bag_name: &str) -> usize {
        let mut cnt : usize = 1;
        if let Some(bags_inside) = self.the_rules.get(bag_name) {
            for bag in bags_inside {
                cnt += bag.cnt * self.nb_of_bags_contained_in(&bag.name);
            }
        }
        cnt
    }

    fn parse_bag_properties (&self, line: &str) -> (String, Vec<CanContain>) {
        // INPUT = vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        // OUTPUT = ("vibrant plum", BapProperties {
        //                              bags_in[0] = CanContain {count = 5, name = "faded blue"}
        //                              bags_in[1] = CanContain {count = 6, name = "dotted black"} }

        let name_props : Vec<&str>= line.split("bags contain").collect();
        let bag_name = name_props[0].trim_end();
        let contain_bags : Vec<&str> = name_props[1].split(",").collect();
        let mut contains = Vec::new();
        for bag in contain_bags {
            let (cnt, name) = Rules::extract_bag_info(bag);
            contains.push(CanContain{cnt, name});
        }
        (String::from(bag_name), contains)
    }

    fn compute_contained_in(&mut self) {
        // Example with = vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        // INTERNAL UPDATE 1 = add in "faded blue" bag properties that it's contained in "vibrant plum"
        // INTERNAL UPDATE 2 = add in "dotted black" bag properties that it's contained in "vibrant plum"

        // for each bag in `the_rules`, walk the `CanContain` vec, and insert its name in the appropriate `contained_in` vec
        for bag_name in self.the_rules.keys() {
            let contains = self.the_rules.get(bag_name).unwrap();
            for c in contains {
                if let Some(target) = self.contained_in.get_mut(&c.name) {
                    target.push(bag_name.to_string());
                    continue;
                }
                else {
                    let mut first_bag = Vec::new();
                    first_bag.push(bag_name.to_string());
                    self.contained_in.insert(c.name.to_string(), first_bag);
                }
            }
        }
    }

    fn extract_bag_info(input : &str) -> (usize, String) {
        // split into words "5 faded blue bags" -> "5", "faded", "blue", "bags".
        let mut cnt = 0;
        let mut name = String::from("");

        let words : Vec<&str> = input.split_whitespace().collect();
        let parsed_cnt = words.first().unwrap().parse::<usize>();
        if parsed_cnt.is_ok() {
            cnt = parsed_cnt.unwrap(); // get the number of bags
            for i in 1..words.len() - 1 { // retrieve the bag name, without the "bag(s)" suffix
                name = name + words[i] + " ";
            }
            name = name.trim_end().to_string();
        }
        (cnt, name)
    }
}
