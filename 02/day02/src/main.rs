use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(PartialEq, Eq)]
enum ItemType {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq)]
enum Points {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

struct Item {
    own_type: ItemType,
    wins_against: ItemType,
    input_keys: [String; 2],
    value: i32,
}

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("No file provided. Please specify file name as command line parameter.");

    let task = env::args()
        .nth(2)
        .expect(
            "Which task should be run? Please specify task number as command line parameter. [1,2]",
        )
        .parse::<i32>()
        .unwrap();
    // INPUT
    let actions: [Item; 3] = setup_actions();
    let game_plan = read_input(file_path, &actions, task).expect("Could not parse file.");
    process_game_plan(game_plan)
}

fn setup_actions() -> [Item; 3] {
    let rock = Item {
        own_type: ItemType::Rock,
        wins_against: ItemType::Scissors,
        input_keys: ["A".to_string(), "X".to_string()],
        value: 1,
    };
    let paper = Item {
        own_type: ItemType::Paper,
        wins_against: ItemType::Rock,
        input_keys: ["B".to_string(), "Y".to_string()],
        value: 2,
    };
    let scissors = Item {
        own_type: ItemType::Scissors,
        wins_against: ItemType::Paper,
        input_keys: ["C".to_string(), "Z".to_string()],
        value: 3,
    };
    return [rock, paper, scissors];
}

fn read_input(
    file_path: String,
    actions: &[Item; 3],
    task: i32,
) -> Result<Vec<Vec<&Item>>, Box<dyn std::error::Error>> {
    println!("Reading File {file_path}");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut games = Vec::new();
    for line in reader.lines() {
        games.push(map_game_round(line, actions, task));
    }
    Ok(games)
}

fn map_game_round(
    line: Result<String, std::io::Error>,
    actions: &[Item; 3],
    task: i32,
) -> Vec<&Item> {
    let mut game: Vec<&Item> = Vec::new();
    if task == 1 {
        for split in line.as_ref().unwrap().to_string().split(' ') {
            game.push(map_input_to_action((split).to_string(), actions).unwrap());
        }
    } else if task == 2 {
        let mut splitting = line.as_ref().unwrap().to_string();
        let mut split = splitting.split(' ');
        let enemy_action = map_input_to_action(split.next().unwrap().to_string(), actions).unwrap();
        game.push(enemy_action);
        let own_action =
            determine_action_to_play(enemy_action, split.next().unwrap().to_string(), actions);
        game.push(own_action.unwrap());
    }
    return game;
}

fn determine_action_to_play<'a>(
    enemy_action: &'a Item,
    own_action: String,
    actions: &'a [Item; 3],
) -> Option<&'a Item> {
    let mut possible_actions: Vec<(&Item, Points)> = Vec::new();
    for action in actions {
        possible_actions.push((action, check_if_won(enemy_action, action)));
    }
    if own_action == "X" {
        return Some(
            possible_actions
                .iter()
                .find(|(_, score)| *score == Points::Loss)
                .unwrap()
                .0,
        );
    } else if own_action == "Y" {
        return Some(
            possible_actions
                .iter()
                .find(|(_, score)| *score == Points::Draw)
                .unwrap()
                .0,
        );
    } else if own_action == "Z" {
        return Some(
            possible_actions
                .iter()
                .find(|(_, score)| *score == Points::Win)
                .unwrap()
                .0,
        );
    } else {
        return None;
    }
}

fn map_input_to_action(input: String, actions: &[Item; 3]) -> Option<&Item> {
    let parsed_action: &Item;
    for action in actions {
        if action.input_keys.contains(&input) {
            parsed_action = action;
            return Some(parsed_action);
        }
    }
    return None;
}

fn check_if_won(a: &Item, b: &Item) -> Points {
    if a.wins_against == b.own_type {
        return Points::Loss;
    } else if b.wins_against == a.own_type {
        return Points::Win;
    } else {
        return Points::Draw;
    }
}

fn process_game_plan(game_plan: Vec<Vec<&Item>>) {
    let mut score: i32 = 0;
    for game in game_plan {
        score += check_if_won(game[0], game[1]) as i32 + game[1].value;
    }
    println!("Final Score equals: {:?}", score);
}
