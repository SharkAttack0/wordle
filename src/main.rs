#![allow(unused)]
use core::panic;
use std::collections::HashMap;
use std::io;
use std::fs;
use std::path::Path;
use std::vec;
use rand::*;
use strum::*;
use colored::*;

const CHARS_ALLOWED_ENGLISH: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
const CHARS_ALLOWED_BULGARIAN:[char; 30] = ['а','б','в','г','д','е','ж','з','и','й','к','л','м','н','о','п','р','с','т','у','ф','х','ц','ч','ш','щ','ъ','ь','ю','я'];
const CHARS_ORDER_ENGLISH: [char; 28] = ['q','w','e','r','t','y','u','i','o','p','\n','a','s','d','f','g','h','j','k','l','\n','z','x','c','v','b','n','m'];
const CHARS_ORDER_BULGARIAN: [char; 32] = ['ч','я','в','е','р','т','ъ','у','и','о','п','ш','щ','ю','\n','а','с','д','ф','г','х','й','к','л','\n','з','ь','ц','ж','б','н','м'];

//find better dictionaries
//refractor
//color states of chars

//current problem: need a way to save CorPos and IncorPos char's state with each user guess
//the point is to have a history of the player's guesses so that they can be color printed
//also, maybe find a better way to print/save colored strings/chars

#[derive(Debug, PartialEq, EnumIter)]
enum CharState {
    Unused,
    UsedCorPos,
    UsedIncorPos,
    UsedNotInWordle,
}
enum Language {
    English,
    Bulgarian,
}
struct LanguageSpecs<'a> {
    language: Language,
    dictionary_path: &'a Path,
    allowed_chars: Vec<char>,
    chars_order: Vec<char>,
}

fn main() {
    //choose language

    let lang_spec_en = LanguageSpecs {
        language: Language::English,
        dictionary_path: Path::new("src/english_dictionary.txt"),
        allowed_chars: CHARS_ALLOWED_ENGLISH.to_vec(),
        chars_order: CHARS_ORDER_ENGLISH.to_vec(),
    };

    let lang_spec_bul = LanguageSpecs {
        language: Language::Bulgarian,
        dictionary_path: Path::new("src/bulgarian_dictionary.txt"),
        allowed_chars: CHARS_ALLOWED_BULGARIAN.to_vec(),
        chars_order: CHARS_ORDER_BULGARIAN.to_vec(),
    };

    let game_lang_spec = lang_spec_en;

    //read file
    let words: String = match fs::read_to_string(game_lang_spec.dictionary_path) {
        Ok(content) => content.to_lowercase(),
        Err(e) => panic!("File not found {}", e),
    };

    let mut words_seperate:Vec<String> = vec![];
    let mut temp_word: Vec<char> = vec![];
    let mut illegal_chars: Vec<char> = vec![];
    for char in words.chars() {
        
        if char == '\n' {
            //if temp_word.len() == 5 {
            words_seperate.push(temp_word.iter().collect());
            //}
            temp_word = Vec::new();
        } else {
            temp_word.push(char);
            
            //check if dictionary contains illegal characters
            if !game_lang_spec.allowed_chars.contains(&char) && !illegal_chars.contains(&char) {
                println!("WARNING: DICTIONARY CONTAINS ILLEGAL CHARACTER!: \"{char}\"");
                illegal_chars.push(char);
            }
        }
         
    }

    let mut words_of_len: Vec<&String> = vec![];

    for word in words_seperate.iter() {
        if word.len() == 5 {
            words_of_len.push(word);
        }
    }

    let mut rng = rand::thread_rng();
    let mut play_again:bool = true;
    let guess_max_number = 6;

    //main loop
    loop { 
        if play_again == false {
            break;
        }
        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let mut guess_counter = 0;
        let wordle = words_of_len[rng.gen_range(0..words_of_len.len())];
        
        
        let mut chars_state: HashMap<char, CharState> = HashMap::new();

        //initialize chars with "unused" state
        for char in game_lang_spec.allowed_chars.iter() {
            chars_state.insert(*char, CharState::Unused);
        }
        let mut guess_all: Vec<String> = vec![];
        let mut chars_state_history: HashMap<&char, &CharState> = HashMap::new();
        //game loop
        loop {

            //prints all guesses
            println!();
            if guess_counter != 0 {
                println!("Guesses so far:");
                for guess in guess_all.iter() {
                    println!("{guess}");
                }
            }

            //last guess warning
            println!();
            if guess_counter == guess_max_number - 1 {
                println!("Last guess!");
            } 

            println!("Guess #{}", guess_counter + 1);

            //get user guess
            let mut user_guess: String;
            loop {
                println!("Enter guess:");
                user_guess = user_input_to_lowercase();
                if user_guess.len() != 5 {
                    println!("Incorrect word length!");
                    continue;
                } else if !words_of_len.contains(&&user_guess) {
                    println!("This word doesn't exist!");
                    continue;
                }
                break;
            }
            guess_all.push(user_guess.clone());
            let mut user_guess_char: Vec<char> = user_guess.chars().collect();

            //check chars of user input and update their state
            let mut chars_all_correct:bool = true;
            
            for (index, char) in  user_guess_char.iter().enumerate() {
                let wordle_chars: Vec<char> = wordle.chars().collect();
                if *char == wordle_chars[index] {
                    //char in correct place
                    chars_state.insert(*char, CharState::UsedCorPos);
                } else {
                    chars_all_correct = false;
                    if wordle_chars.contains(&char) {
                        //char in incorrect place
                        chars_state.insert(*char, CharState::UsedIncorPos);
                    } else {
                        //char not in wordle
                        chars_state.insert(*char, CharState::UsedNotInWordle);
                    }
                }
            }


            //print guess in color
            print_colored(&chars_state, &user_guess_char);



            //check if guess if correct
            if chars_all_correct {
                println!();
                println!("That is the wordle! You win!");
                if !user_input_yes_no_bool("Would you like to play again? y/n:") {
                    play_again = false;
                }
                break;
            }

            print_colored(&chars_state, &game_lang_spec.chars_order);

            let char = "red".red();

            //guesses counter and checker
            guess_counter += 1;
            if guess_counter == guess_max_number {
                println!();
                println!("You ran out of guesses! You lost!");
                println!("The wordle was {}", wordle.to_uppercase().green().bold());
                if !user_input_yes_no_bool("Would you like to play again? y/n:") {
                    play_again = false;
                }
                break;
            }
        }
    }

    println!("Bye!");
}

fn user_input_yes_no_bool(line: &str) -> bool {
    //returns true if user says yes, asks again if input is invalid
    loop {
        println!("{line}");
        let user_input = user_input_to_lowercase();
        if user_input == String::from("y") || user_input == String::from("yes") {
            //user typed yes
            return true;
        } else if user_input == String::from("n") || user_input == String::from("no") {
            //user typed no
            return false;
        } else {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Invalid input!");
            continue;
        }
    }
}

fn user_input_to_lowercase() -> String {
    let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => (),
            Err(err) => println!("Error! {err}"),
        }
        user_input = String::from(user_input.trim());
        user_input.to_lowercase()
}

fn print_colored(chars_state: &HashMap<char, CharState>, chars_color: &Vec<char>) {
    println!();
    
        for char in chars_color {
            if *char == '\n' {
                println!();
            } else {
                match chars_state.get(&char) {
                    Some(state) => {
                        match state {
                            CharState::UsedCorPos => print!("{} ", char.to_string().green().bold().italic()),
                            CharState::UsedIncorPos => print!("{} ", char.to_string().yellow().bold().italic()),
                            CharState::UsedNotInWordle => print!("{} ", char.to_string().red().bold().italic()),
                            CharState::Unused => print!("{} ", char.to_string().truecolor(36, 36, 36).bold().italic()),
                        }
                    },
                    None => panic!("user_guess contains char that is not allowed!"),
                }
            }
        }
    println!();
}