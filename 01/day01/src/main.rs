use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No file provided. Please specify file name as command line parameter.");
    // INPUT
    let elfes_list = read_input(file_path).expect("Could not parse file.");

    // TASK 1: CALCULATE MAX CALORIES ON ONE ELF
    let mut calories_per_elf: Vec<i32> = Vec::new();
    for elf in elfes_list.iter() {
        calories_per_elf.push(sum_up_elf(elf));
    }
    println!(
        "Maximum calories on one elf: {:?}",
        get_top_x_elves(calories_per_elf.clone(), 1)
            .iter()
            .sum::<i32>()
    );

    // TASK 2: CALCULATE MAX CALORIES AMONG THREE ELVES
    let top_x_elves: Vec<i32> = get_top_x_elves(calories_per_elf.clone(), 3);
    println!(
        "Maximum calories on top three elfs: {:?}",
        top_x_elves.iter().sum::<i32>()
    );
}

fn read_input(file_path: String) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    println!("Reading File {file_path}");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut elves_list = Vec::new();
    let mut elf: Vec<String> = Vec::new();
    for line in reader.lines() {
        if line.as_ref().unwrap() == "" {
            elves_list.push(elf);
            elf = Vec::new();
        } else {
            elf.push(line.as_ref().unwrap().to_string());
        }
    }
    Ok(elves_list)
}

// returns the sum of calories that one elf carries
fn sum_up_elf(elf: &Vec<String>) -> i32 {
    let mut calories = 0;
    for calory_item in elf.iter() {
        calories += calory_item.parse::<i32>().unwrap();
    }
    return calories;
}

// returns a vector of the x highest values present the provided vector
fn get_top_x_elves(mut calories_per_elf: Vec<i32>, x: i32) -> Vec<i32> {
    let mut top_x_elves: Vec<i32> = Vec::new();
    for _i in 0..x {
        let best_elf = *calories_per_elf.iter().max().unwrap();
        top_x_elves.push(best_elf);
        calories_per_elf.remove(
            calories_per_elf
                .iter()
                .position(|y| *y == best_elf)
                .unwrap(),
        );
    }
    return top_x_elves;
}
