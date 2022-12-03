use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const UPPERCASE_ASCII_A: i32 = 65;
const LOWERCASE_ASCII_A: i32 = 97;
const UPPERCASE_START_INDEX: i32 = 27;
const LOWERCASE_START_INDEX: i32 = 1;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No file provided. Please specify file name as command line parameter.");
    let input = read_lines(file_path).expect("File could not be parsed.");

    // TASK 1
    let backpacks = init_backpack_compartments(input.clone());
    let duplicate_items = get_duplicate_items(backpacks);
    let duplicate_item_priorities = get_summed_item_priorities(duplicate_items);
    println!("{:?}", duplicate_item_priorities);

    // TASK 2
    let groups = init_groups(input.clone());
    let common_items = get_common_items_for_groups(groups);
    let common_items_priorities = get_summed_item_priorities(common_items);
    println!("{:?}", common_items_priorities);
}

// TASK 1

fn init_backpack_compartments(backpacks: Vec<String>) -> Vec<(String, String)> {
    let mut backpacks_comparted = Vec::new();
    for backpack in backpacks {
        backpacks_comparted.push(split_string_in_half(backpack));
    }
    return backpacks_comparted;
}

fn check_for_duplicate_item((x, y): (String, String)) -> Option<char> {
    for c in x.chars() {
        if y.contains(c) {
            return Some(c);
        }
    }
    return None;
}

fn get_duplicate_items(backpacks: Vec<(String, String)>) -> Vec<char> {
    let mut duplicate_items = Vec::new();
    for backpack in backpacks {
        duplicate_items.push(check_for_duplicate_item(backpack).unwrap());
    }
    return duplicate_items;
}

fn get_summed_item_priorities(duplicate_items: Vec<char>) -> i32 {
    let mut item_priorities = Vec::new();
    for item in duplicate_items {
        item_priorities.push(determine_char_value(item));
    }
    return item_priorities.iter().sum();
}

fn determine_char_value(c: char) -> i32 {
    let mut char_value: i32 = 0;
    if c.is_lowercase() {
        char_value += -LOWERCASE_ASCII_A + LOWERCASE_START_INDEX;
    } else {
        char_value += -UPPERCASE_ASCII_A + UPPERCASE_START_INDEX;
    }
    char_value += c as u32 as i32;
    return char_value;
}

// TASK 2:

fn init_groups(input: Vec<String>) -> Vec<Vec<String>> {
    let mut groups = Vec::new();
    for i in (0..input.len()).step_by(3) {
        let mut group = Vec::new();
        for r in 0..3 {
            group.push(input[i + r].clone());
        }
        groups.push(group);
    }
    return groups;
}

fn get_common_items_for_groups(groups: Vec<Vec<String>>) -> Vec<char> {
    let mut common_items = Vec::new();
    for group in groups {
        common_items.push(get_common_item(group).unwrap());
    }
    return common_items;
}

fn get_common_item(group: Vec<String>) -> Option<char> {
    for c in group[0].chars() {
        if group[1].contains(&c.to_string()) && group[2].contains(&c.to_string()) {
            return Some(c);
        }
    }
    return None;
}

// HELPERS

fn read_lines(file_path: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut content = Vec::new();
    for line in reader.lines() {
        content.push(line.as_ref().unwrap().to_string());
    }
    Ok(content)
}

fn split_string_in_half(string_to_split: String) -> (String, String) {
    let length = string_to_split.chars().count();
    let (x, y) = string_to_split.split_at(length / 2);
    return (x.to_string(), y.to_string());
}
