use std::io::{self, Stdin, Write};

use rand::{thread_rng, RngCore};
use rand::rngs::ThreadRng;

fn main() {

    let mut rng = thread_rng();

    let dictionary = vec!["hello", "World", "hAppY", "lifetime", "parameter", "crustacean", "life"];

    let mut word_goal = roll_new_word(&mut rng, &dictionary);

    let mut replay = true;

    while replay {
        print!("Your Input: ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut user_input).expect("Failed to read User Input");
        println!();

        user_input = clean_word(&user_input);

        if user_input.eq_ignore_ascii_case(&word_goal) {
            println!("Wiiinnn!!");
            replay = ask_for_replay(stdin, &mut user_input);
            if replay {
                word_goal = roll_new_word(&mut rng, &dictionary);
                println!("\nNEW WORD CHOSEN - GOOD LUCK!\n");
            }
        } else {
            println!("{} and {} are different lol", &word_goal, &user_input);
        }
    }

    println!("Bye, have a nice day!");

}

fn roll_new_word(rng: &mut ThreadRng, dictionary: &Vec<&str>) -> String {
    let rolled_value = rng.next_u64() as usize;

    clean_word(dictionary[rolled_value % dictionary.len()])
}

fn clean_word(word: &str) -> String {
    word.replace(['\r', '\n', '\t', ' '], "")
}

fn ask_for_replay(stdin: Stdin, user_input: &mut String) -> bool {
    println!("Do you want to play again?");
    println!("1. Yes\n2. No");
    stdin.read_line(user_input).expect("Failed to read User input");
    user_input.contains("1")
}
