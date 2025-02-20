use std::error::Error;
use std::{char, io};
use text_to_ascii_art::to_art;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let word = fetch_word().await?;

    let allowed_guesses = word.len() + 1;
    let mut current_guess = 0;

    println!("The hidden word is {} letters long!", word.len());
    println!("You have {} guesses!", allowed_guesses - 1);

    println!(
        "{}",
        get_revealed_word(&word, &(word.chars().map(|_ch| '_').collect::<String>()))
    );

    while current_guess < allowed_guesses {
        let mut guessed_word: String;
        loop {
            guessed_word = prompt_user()?;
            if guessed_word.len() == word.len() {
                break;
            }
            println!("Guess must be of length {}", word.len());
        }

        let revealed_word = get_revealed_word(&word, &guessed_word);

        current_guess += 1;

        println!(
            "{} | {} guesses remaining.",
            revealed_word,
            allowed_guesses - current_guess
        );

        if guessed_word == word {
            println!("{}", to_art("YOU WON!".to_string(), "DEFAULT", 0, 0, 0,)?);
            println!("{}", to_art(word.to_string(), "DEFAULT", 0, 0, 0,)?);
            break;
        }

        if current_guess == allowed_guesses {
            println!("{}", to_art("GAME OVER!".to_string(), "DEFAULT", 0, 0, 0,)?);
            println!(
                "{}",
                to_art("THE WORD WAS...".to_string(), "DEFAULT", 0, 0, 0,)?
            );
            println!("{}", to_art(word.to_string(), "DEFAULT", 0, 0, 0,)?);
        }
    }

    Ok(())
}

fn get_revealed_word(word: &str, guessed_word: &str) -> String {
    let mut letters_in_guess: Vec<char> = Vec::from_iter(guessed_word.chars());
    let mut letters_in_word: Vec<char> = Vec::from_iter(word.chars());

    let revealed_word: String = word
        .chars()
        .zip(guessed_word.chars())
        .map(|(correct_character, guessed_character)| {
            if correct_character == guessed_character {
                remove_first(&mut letters_in_guess, correct_character);
                remove_first(&mut letters_in_word, correct_character);
                correct_character
            } else {
                '_'
            }
        })
        .collect();

    revealed_word
        .chars()
        .enumerate()
        .map(|(index, current_character)| {
            if current_character == '_' {
                let guessed_char = guessed_word.chars().nth(index).unwrap();

                if letters_in_word.contains(&guessed_char)
                    && letters_in_guess.contains(&guessed_char)
                {
                    remove_first(&mut letters_in_guess, guessed_char);
                    remove_first(&mut letters_in_word, guessed_char);

                    '?'
                } else {
                    current_character
                }
            } else {
                current_character
            }
        })
        .collect()
}

fn remove_first(vec: &mut Vec<char>, element: char) {
    if let Some(index) = vec.iter().position(|value| *value == element) {
        vec.remove(index);
    }
}

fn prompt_user() -> Result<String, Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

async fn fetch_word() -> Result<String, Box<dyn Error>> {
    let url = "https://random-word-api.herokuapp.com/word?length=5";
    let response = reqwest::get(url).await?;
    let word = response
        .json::<Vec<String>>()
        .await?
        .into_iter()
        .next()
        .unwrap();

    Ok(word)
}
