use std::fs;
use std::time::Instant;
use regex::Regex;

fn main() {
    let start = Instant::now();

    let tst_tickets = TicketValidator::from_file("example.txt");
    let mut tickets = TicketValidator::from_file("input.txt");

    // part 1
    let mut tst_scanning_error_rate = 0;
    for nearby_ticket in &tst_tickets.nearby_tickets {
        tst_scanning_error_rate += tst_tickets.invalid_field(nearby_ticket);
    }
    assert_eq!(71, tst_scanning_error_rate);
    let mut scanning_error_rate = 0;
    for nearby_ticket in &tickets.nearby_tickets {
        scanning_error_rate += tickets.invalid_field(nearby_ticket);
    }
    println!("Scanning error rate = {}", scanning_error_rate);

    // part2
    let mut tst2_tickets = TicketValidator::from_file("example2.txt");
    assert_eq!(vec!["row", "class", "seat"], tst2_tickets.find_order());

    let order = tickets.find_order();
    let mut result = 1;
    let mut field_idx = 0;
    for ordered_field in order {
        if ordered_field.starts_with("departure") {
            result *= tickets.your_ticket.get(field_idx).unwrap();
        }
        field_idx += 1;
    }
    println!("Result = {}", result);

    // exec time
    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

struct FieldRules {
    name: String,
    idx: Vec<usize>,
    min_1: usize,
    max_1: usize,
    min_2: usize,
    max_2: usize
}
impl FieldRules {
    fn is_valid(&self, field: &usize) -> bool {
        return *field >= self.min_1 && *field <= self.max_1 ||
            *field >= self.min_2 && *field <= self.max_2;
    }
}

struct TicketValidator {
    rules: Vec<FieldRules>,
    your_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>
}

impl TicketValidator {
    fn invalid_field(&self, ticket: &Vec<usize>) -> usize {
        // matching any of the rules is enough
        'field_loop: for field in ticket {
            'rule_loop: for rule in &self.rules {
                if rule.is_valid(field) { continue 'field_loop;}
                continue 'rule_loop;
            }
            return *field; // invalid field!
        }
        0 // valid ticket
    }

    fn find_order(&mut self) -> Vec<String> {
        // 1. Keep only the valid tickets
        // 2. For each ticket
        //      for each field
        //         for each rule : is it a valid field?
        //  If no, remove that field index from the list of possible index, for this rule
        // At the end of 2. we should have only 1 rule with only one possible index
        // 3. Then, we must reduce the possible index for all the other rules, starting by the
        //   the most constrained one, ie.e the one with only one possible index.
        // 4. Create a vector containing the fields in order.

        // Step 1. Keep valid tickets only
        let mut valid_tickets = Vec::new();
        for nearby_ticket in &self.nearby_tickets {
            if self.invalid_field(nearby_ticket) == 0 {
                valid_tickets.push(nearby_ticket.clone());
            }
        }

        // Step 2. Compute all the possible field index for each rule
        for ticket in valid_tickets {
            for field_idx in 0..self.rules.len() {
                let field = ticket.get(field_idx).unwrap();
                'rule_loop: for rule_idx in 0..self.rules.len() {
                    let rule = self.rules.get_mut(rule_idx).unwrap();
                    if ! rule.is_valid(field) {
                        rule.idx.retain(|&x| x != field_idx); // remove that field index
                    }
                    continue 'rule_loop;
                }
            }
        }

        // Step 3. Reduce by starting from the rule that has only one possible index, and propagate to other rules
        let mut unvisited_rule :Vec<usize> = (0..self.rules.len()).collect();
        'reduce: loop {
            // There must be at least one rule that have only one possible index. Find it, and remove that field index from all the other rules
            let mut index_to_remove : usize = 0;
            let mut visiting_rule_name = "".to_string();
            let mut visiting_rule_idx = 0;
            'find_most_constrained_rule: for visiting_idx in &unvisited_rule {
                let rule = self.rules.get(visiting_idx.clone()).unwrap();
                if rule.idx.len() == 1 {
                    index_to_remove = rule.idx.first().unwrap().clone();
                    visiting_rule_name = rule.name.clone();
                    visiting_rule_idx = visiting_idx.clone();
                    break 'find_most_constrained_rule;
                }
            }
            unvisited_rule.retain(|&x| x != visiting_rule_idx);

            'edit_other_rules: for rule_idx in 0..self.rules.len() {
                let rule = self.rules.get_mut(rule_idx).unwrap();
                if rule.name == visiting_rule_name { // don't remove it from the reference rule
                    continue 'edit_other_rules;
                }
                else { // remove that field index
                    rule.idx.retain(|&x| x != index_to_remove);
                }
            }
            // loop while there is at least one rule with more than 1 index
            for rule_idx in &unvisited_rule {
                let rule = self.rules.get(rule_idx.clone()).unwrap();
                if rule.idx.len() > 1 {
                    continue 'reduce;
                }
            }
            // if there's only rule with one idx candidate, break
            break 'reduce;
        }

        // Step 4. Create a vector containing the fields in order
        let mut order : Vec<String> = Vec::new();
        // find the rule corresponding to the index from 0 to rules.len()
        for i in 0..self.rules.len() {
            for rule in &self.rules {
                if rule.idx.contains(&i) {
                    order.push(rule.name.clone());
                }
            }
        }
        //println!("order = {:?}", order);
        order
    }

    fn from_file(f_name: &str) -> TicketValidator {
        let str_in = fs::read_to_string(f_name).expect("Error in reading file");
        // split by empty line:
        // 1. Field rules
        // 2. Your ticket
        // 3. Nearby tickets

        let mut rules : Vec<FieldRules> = Vec::new();
        let parts:Vec<&str> = str_in.split("\r\n\r\n").collect();
        let re = Regex::new(r"(\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        let rules_cnt = parts[0].lines().count();
        for rule in parts[0].lines() {
            let name_and_rules:Vec<&str> = rule.split(":").collect();
            if name_and_rules.len() < 2 { break;}

            let name = name_and_rules[0].to_string();
            let cap = re.captures(name_and_rules[1]).unwrap();
            let min_1 = cap[1].parse::<usize>().unwrap();
            let max_1 = cap[2].parse::<usize>().unwrap();
            let min_2 = cap[3].parse::<usize>().unwrap();
            let max_2 = cap[4].parse::<usize>().unwrap();
            let idx : Vec<usize> = (0..rules_cnt).collect();
            rules.push(FieldRules{name, idx, min_1, max_1, min_2, max_2});
        }

        let your_ticket_str = parts[1].lines().nth(1).unwrap();
        let your_ticket = TicketValidator::parse_ticket(your_ticket_str);

        let mut nearby_tickets : Vec<Vec<usize>> = Vec::new();
        for l in parts[2].lines() {
            if l.starts_with("nearby") {continue;}
            nearby_tickets.push(TicketValidator::parse_ticket(l));
        }

        TicketValidator {
            rules,
            your_ticket,
            nearby_tickets
        }
    }

    fn parse_ticket(str_in: &str) -> Vec<usize> {
        let mut ticket : Vec<usize> = Vec::new();
        let fields : Vec<&str> = str_in.split(",").collect();
        for field in fields {
            ticket.push(field.parse::<usize>().unwrap());
        }
        ticket
    }
}