use std::{collections::HashMap, fs};

fn get_type_score(play: &str) -> u32 {
    match play {
        "rock" => 1,
        "paper" => 2,
        "scissors" => 3,
        _ => 0,
    }
}

fn get_result_score(play: &str, opp_play: &str) -> u32 {
    match (play, opp_play) {
        ("rock", "rock") => 3,
        ("rock", "paper") => 0,
        ("rock", "scissors") => 6,
        ("paper", "rock") => 6,
        ("paper", "paper") => 3,
        ("paper", "scissors") => 0,
        ("scissors", "rock") => 0,
        ("scissors", "paper") => 6,
        ("scissors", "scissors") => 3,
        _ => 0,
    }
}

fn get_my_play(result: &str, opp_play: &str) -> String {
    match (result, opp_play) {
        ("win", "rock") => String::from("paper"),
        ("win", "paper") => String::from("scissors"),
        ("win", "scissors") => String::from("rock"),
        ("lose", "rock") => String::from("scissors"),
        ("lose", "paper") => String::from("rock"),
        ("lose", "scissors") => String::from("paper"),
        ("draw", "rock") => String::from("rock"),
        ("draw", "paper") => String::from("paper"),
        ("draw", "scissors") => String::from("scissors"),
        _ => String::new(),
    }
}

fn main() {
    let translation = HashMap::from([
        ("A", "rock"),
        ("B", "paper"),
        ("C", "scissors"),
        ("X", "rock"),
        ("Y", "paper"),
        ("Z", "scissors"),
    ]);
    let my_play_map = HashMap::from([("X", "lose"), ("Y", "draw"), ("Z", "win")]);

    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");
    let mut total_score = 0;
    let mut total_r2_score = 0;

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        let plays: Vec<&str> = line.split(" ").collect();
        let my_play = translation[plays[1].trim()];
        let opp_play = translation[plays[0].trim()];
        let my_game_score = get_type_score(my_play) + get_result_score(my_play, opp_play);

        let my_r2_result = my_play_map[plays[1].trim()];
        let my_r2_play = get_my_play(my_r2_result, opp_play);
        let my_r2_score = get_type_score(&my_r2_play[..]) + 
                          get_result_score(&my_r2_play[..], opp_play);

        total_score += my_game_score;
        total_r2_score += my_r2_score;
    }

    println!("Answer 1: {total_score}");
    println!("Answer 2: {total_r2_score}");
}
