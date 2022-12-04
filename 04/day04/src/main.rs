use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No file provided. Please specify file name as command line parameter.");
    let input = read_lines(file_path).expect("File could not be parsed.");
    let assignments = parse_assignments(input);
    let contained_assignments = get_fully_contained_assignments(&assignments);
    println!("{:?}", contained_assignments.len());
    let overlapping_assignments = get_overlapping_assignments(&assignments);
    println!("{:?}", overlapping_assignments.len());
}

fn parse_assignments(input: Vec<String>) -> Vec<Vec<(i32, i32)>> {
    let mut assignments = Vec::new();
    for pair in input {
        let split = pair.split(",");
        let mut assignment = Vec::new();
        for elf in split {
            let mut task = elf.split("-");
            assignment.push((
                task.next().unwrap().parse::<i32>().unwrap(),
                task.next().unwrap().parse::<i32>().unwrap(),
            ));
        }
        assignments.push(assignment);
    }
    return assignments;
}

fn get_fully_contained_assignments(assignments: &Vec<Vec<(i32, i32)>>) -> Vec<&Vec<(i32, i32)>> {
    let mut contained_assignments = Vec::new();
    for pair in assignments {
        if check_if_contained(pair[0], pair[1]) {
            contained_assignments.push(pair);
        }
    }
    return contained_assignments;
}

fn get_overlapping_assignments(assignments: &Vec<Vec<(i32, i32)>>) -> Vec<&Vec<(i32, i32)>> {
    let mut overlapping_assignments = Vec::new();
    for pair in assignments {
        if check_if_overlap(pair[0], pair[1]) {
            overlapping_assignments.push(pair);
        }
    }
    return overlapping_assignments;
}

fn check_if_contained(x: (i32, i32), y: (i32, i32)) -> bool {
    if (x.0 >= y.0 && x.1 <= y.1) || (y.0 >= x.0 && y.1 <= x.1) {
        return true;
    }
    return false;
}

fn check_if_overlap(x: (i32, i32), y: (i32, i32)) -> bool {
    if (x.1 < y.0) || (x.0 > y.1) {
        return false;
    }
    return true;
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
