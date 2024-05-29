use colored::*;
use core::panic;
use rand::*;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::vec;
use strum::*;


const CHARS_ALLOWED_ENGLISH: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
const CHARS_ALLOWED_BULGARIAN: [char; 30] = [
    'а', 'б', 'в', 'г', 'д', 'е', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п', 'р', 'с', 'т',
    'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ь', 'ю', 'я',
];
const CHARS_ORDER_ENGLISH: [char; 28] = [
    'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '\n', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k',
    'l', '\n', 'z', 'x', 'c', 'v', 'b', 'n', 'm',
];
const CHARS_ORDER_BULGARIAN: [char; 32] = [
    'ч', 'я', 'в', 'е', 'р', 'т', 'ъ', 'у', 'и', 'о', 'п', 'ш', 'щ', 'ю', '\n', 'а', 'с', 'д', 'ф',
    'г', 'х', 'й', 'к', 'л', '\n', 'з', 'ь', 'ц', 'ж', 'б', 'н', 'м',
];


//bulgarian 'ш','щ' debugging conclusion - error "stream did not contain valid UTF-8" is not because of
//the bulgarian chars, but rather the backspace being pressed when language is bulgarian
//could be wsl terminal's fault, idk if program can handle that at all 

//edit dictionaries
//full bulgarian language support

//when a char is used twice in a guess and one of the chars is updated, the other gets wrongly updated as well


#[derive(Debug, PartialEq, EnumIter, Clone, Copy)]
enum CharState {
    Unused,
    UsedCorPos,
    UsedIncorPos,
    UsedNotInWordle,
}
#[derive(Debug, PartialEq, Eq, Hash)]
enum Language {
    English,
    Bulgarian,
}

struct Messages {
    user_lost: String,
    user_won: String,
    guess_num: String,
    wordle_was: String,
    play_again_query: String,
    guess_query: String,
    guesses_so_far: String,
    goodbye: String,
    warn_dict_illegal: String,
    guess_last: String,
    dict_not_found: String,
    err: String, 
    err_invalid_input: String,
    err_word_lenght: String,
    err_word_not_exist: String,
    user_input_yes: String,
    user_input_no: String,

}

struct LanguageSpecs<'a> {
    dictionary_path: &'a Path,
    allowed_chars: Vec<char>,
    chars_order: Vec<char>,
    messages: Messages,
}

fn main() {
    let msg_en: Messages = Messages {
        user_lost: String::from("You ran out of guesses! You lost!"),
        user_won: String::from("Congradulation! You won!"),
        guess_num: String::from("Guess #"),
        wordle_was: String::from("The wordle was"),
        play_again_query: String::from("Would you like to play again? y/n"),
        guess_query: String::from("Input guess:"),
        guesses_so_far: String::from("Your guesses so far:"),
        goodbye: String::from("Goodbye!"),
        warn_dict_illegal: String::from("WARNING: DICTIONARY CONTAINS ILLEGAL CHARACTER!:"),
        guess_last: String::from("Last guess!"),
        dict_not_found: String::from("Dictionary not found!"),
        err: String::from("Error!"), 
        err_invalid_input: String::from("Invalid input!"),
        err_word_lenght: String::from("The word's length is incorrect!"),
        err_word_not_exist: String::from("The word doesn't exist!"),
        user_input_yes: String::from("yes"),
        user_input_no: String::from("no"),
    };

    let msg_bul: Messages = Messages {
        user_lost: String::from("Докадките ти свършиха! Ти загуби!"),
        user_won: String::from("Поздравления! Ти спечели!"),
        guess_num: String::from("Догадка #"),
        wordle_was: String::from("Думата беше"),
        play_again_query: String::from("Искаш ли да играеш пак? д/н"),
        guess_query: String::from("Въведи догадка:"),
        guesses_so_far: String::from("Твоите догадки дотук:"),
        goodbye: String::from("Довиждане!"),
        warn_dict_illegal: String::from("ВНИМАНИЕ: РЕЧНИКЪТ СЪДЪРЖА НЕЗАКОННИ СИМВОЛИ!:"),
        guess_last: String::from("Последна догадка!"),
        dict_not_found: String::from("Речник не е намерен!"),
        err: String::from("Грешка!"), 
        err_invalid_input: String::from("Невалидно въвеждане!"),
        err_word_lenght: String::from("Дължината на думата е неправилна!"),
        err_word_not_exist: String::from("Тази дума не съществува!"),
        user_input_yes: String::from("да"),
        user_input_no: String::from("не"),
    };

    let lang_spec_en = LanguageSpecs {
        dictionary_path: Path::new("src/english_dictionary.txt"),
        allowed_chars: CHARS_ALLOWED_ENGLISH.to_vec(),
        chars_order: CHARS_ORDER_ENGLISH.to_vec(),
        messages: msg_en,
    };

    let lang_spec_bul = LanguageSpecs {
        dictionary_path: Path::new("src/bulgarian_dictionary.txt"),
        allowed_chars: CHARS_ALLOWED_BULGARIAN.to_vec(),
        chars_order: CHARS_ORDER_BULGARIAN.to_vec(),
        messages: msg_bul,
    };

    //choose language
    let chosen_lang: Language = query_lang();

    let game_lang_spec :LanguageSpecs;
    let wordle_len:usize;

    match chosen_lang {
        Language::English => {
            game_lang_spec = lang_spec_en;
            wordle_len = 5;
        },
        Language::Bulgarian => {
            game_lang_spec = lang_spec_bul;
            wordle_len = 10;
        },
    }


    //read file
    let words: String = match fs::read_to_string(game_lang_spec.dictionary_path) {
        Ok(content) => content.to_lowercase(),
        Err(err) => panic!("{} {err}", game_lang_spec.messages.dict_not_found),
    };

    let mut words_seperate: Vec<String> = vec![];
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
                println!("{} \"{char}\"", game_lang_spec.messages.warn_dict_illegal);
                illegal_chars.push(char);
            }
        }
    }

    let mut words_of_len: Vec<&String> = vec![];

    for word in words_seperate.iter() {
        if word.len() == wordle_len {
            words_of_len.push(word);
        }
    }

    let mut rng = rand::thread_rng();
    let mut play_again: bool = true;
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
        let mut char_history: Vec<(Vec<char>, HashMap<char, CharState>)> = vec![];
        //game loop
        loop {
            //prints all guesses
            println!();
            //last guess warning
            println!();
            if guess_counter == guess_max_number - 1 {
                println!("{}", game_lang_spec.messages.guess_last);
            }

            println!("{}{}", game_lang_spec.messages.guess_num, guess_counter + 1);

            //get user guess
            let mut user_guess: String;
            loop {
                println!("{}", game_lang_spec.messages.guess_query);
                user_guess = user_input_to_lowercase();

                if user_guess.len() != wordle_len {
                    println!("{}", game_lang_spec.messages.err_word_lenght);
                    println!("{}", user_guess.len());
                    continue;
                } else if !words_of_len.contains(&&user_guess) {
                    println!("{}", game_lang_spec.messages.err_word_not_exist);
                    continue;
                }
                break;
            }
            guess_all.push(user_guess.clone());
            let user_guess_char: Vec<char> = user_guess.chars().collect();

            //check chars of user input and update their state
            let mut chars_all_correct: bool = true;

            for (index, char) in user_guess_char.iter().enumerate() {
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
            char_history.push((user_guess_char.clone(), chars_state.clone()));

            //check if guess if correct
            if chars_all_correct {
                println!();
                println!("{}", game_lang_spec.messages.user_won);
                if !user_input_yes_no_bool(&game_lang_spec) {
                    play_again = false;
                }
                break;
            }
            
            //print chars' states in keyboard layout
            println!();
            print_colored(&chars_state, &game_lang_spec.chars_order, true);
            //print guesses
            println!("\n{}", game_lang_spec.messages.guesses_so_far);
            for history in char_history.iter() {
                print_colored(&history.1, &history.0, false);
            }

            //guesses counter and checker
            guess_counter += 1;
            if guess_counter == guess_max_number {
                println!();
                println!("{}", game_lang_spec.messages.user_lost);
                println!("{} {}",game_lang_spec.messages.wordle_was, wordle.to_uppercase().green().bold());
                if !user_input_yes_no_bool(&game_lang_spec) {
                    play_again = false;
                }
                break;
            }
        }
    }

    println!("{}", game_lang_spec.messages.goodbye);
}

fn user_input_yes_no_bool(game_lang_spec: &LanguageSpecs) -> bool {
    //returns true if user says yes, asks again if input is invalid
    loop {
        println!("{}", game_lang_spec.messages.play_again_query);
        let user_input = user_input_to_lowercase();
        if user_input == game_lang_spec.messages.user_input_yes.chars().next().unwrap().to_string() || user_input == game_lang_spec.messages.user_input_yes {
            //user typed yes
            return true;
        } else if user_input == game_lang_spec.messages.user_input_no.chars().next().unwrap().to_string() || user_input == game_lang_spec.messages.user_input_no {
            //user typed no
            return false;
        } else {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("{}", game_lang_spec.messages.err_invalid_input);
            continue;
        }
    }
}

fn user_input_to_lowercase() -> String {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => (),
        Err(err) => {
            println!("Error! {err}");
        }
    }
    user_input = String::from(user_input.trim());
    user_input.to_lowercase()
}

fn print_colored(chars_state: &HashMap<char, CharState>, chars_to_color: &Vec<char>, space: bool) {
    for char in chars_to_color {
        if *char == '\n' {
            println!();
        } else {
            match chars_state.get(&char) {
                Some(state) => print_char_state_to_color(state, char),
                None => panic!("user_guess contains char that is not allowed!"),
            }
            if space {
                print!(" ");
            }
        }
    }
    println!();
}

fn print_char_state_to_color(state: &CharState, char: &char) {
    match state {
        CharState::UsedCorPos => {
            print!("{}", char.to_string().green().bold().italic())
        }
        CharState::UsedIncorPos => {
            print!("{}", char.to_string().yellow().bold().italic())
        }
        CharState::UsedNotInWordle => {
            print!("{}", char.to_string().red().bold().italic())
        }
        CharState::Unused => print!("{}", char.to_string().truecolor(72, 72, 72).bold().italic()),
    }
}

fn query_lang() -> Language {
    loop {
        println!("1.English");
        println!("2.Български");
        println!("Choose Language: ");
        let user_input = user_input_to_lowercase();
        if user_input == String::from("1") || user_input == String::from("english") {
            return Language::English;
        }

        if user_input == String::from("2") || user_input == String::from("български") {
            return Language::Bulgarian;
        }
        //exception to printing lang specific message: lang isn't being chosen yet
        println!("Error: Invalid input!");
    }
}
