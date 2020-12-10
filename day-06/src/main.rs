use std::time::Instant;
use std::fs;

fn main() {
    let start = Instant::now();

    let str_test = fs::read_to_string("example.txt").expect("Error in reading file");
    let all_groups = split_by_empty_line(&str_test);
    let sum_pt1 = sum_part1(&all_groups);
    assert_eq!(sum_pt1, 11);
    let sum_pt2 = sum_part2(&all_groups);
    assert_eq!(sum_pt2, 6);

    let str_in = fs::read_to_string("input.txt").expect("Error in reading file");
    let all_groups = split_by_empty_line(&str_in);
    let sum_pt1 = sum_part1(&all_groups);
    println!("Sum part1 = {}", sum_pt1);
    let sum_pt2 = sum_part2(&all_groups);
    println!("Sum part2 = {}", sum_pt2);

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn sum_part1(groups: &Vec<String>) -> usize {
    let mut result = 0;
    for this_group in groups {
        result += cnt_all_answers_in(this_group);
    }
    result
}

fn cnt_all_answers_in(group: &String) -> usize {
    let mut result = Vec::new();
    for each_one in group.lines(){
        for answers in each_one.chars() {
            if result.contains(&answers) {
                continue;
            }
            result.push(answers);
        }
    }
    result.len()
}

fn sum_part2(groups: &Vec<String>) -> usize {
    let mut result = 0;
    for this_group in groups {
        result += cnt_common_answer_in(this_group);
    }
    result
}

fn cnt_common_answer_in(group: &String) -> usize {
    let mut common_answers = "abcdefghijklmnopqrstuvwxyz".to_string();
    for each_one_answer in group.split_whitespace() {
        common_answers = strip_characters(&common_answers, each_one_answer);
    }
    common_answers.len()
}

fn strip_characters(original : &str, to_keep : &str) -> String {
    original.chars().filter(|&c| to_keep.contains(c)).collect()
}

fn split_by_empty_line(input: &String) -> Vec<String> {
    let mut groups = Vec::new();

    let mut new_group = "".to_string();
    // Fill answers per group from input
    for line in input.lines() {
        // while no empty line, concatenate the string
        if line.len() > 0 {
            new_group = new_group + line + "\n";
        }
        else {
            groups.push(new_group);  // group is complete.
            new_group = "".to_string(); // reset the string (new group)
        }
    }
    // for loop is finished, put the latest in group in the vector
    groups.push(new_group);
    groups
}
