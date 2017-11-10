extern crate rand;

use std::io::prelude::*;
use std::io::{self, Write};
use std::fs::File;
use rand::Rng;


fn setup() -> String {
    let mut rng = rand::thread_rng();
    let idx_word = rng.gen_range(0, 974);

    let mut f = File::open("1-1000.txt").expect("file not found");
    let mut f_buf = String::new();
    f.read_to_string(&mut f_buf);
    String::from(f_buf.lines().nth(idx_word).expect("bad"))
}

fn index_in_string(c: char, s: &str) -> isize {
    for (idx, check) in s.chars().enumerate() {
        if c == check {
            return idx as isize;
        }
    }
    -1
}

fn get_char_input() -> char {
    print!("Guess a character> ");
    io::stdout().flush().expect("Failed to flush");

    let mut input_buff = String::new();

    io::stdin().read_line(&mut input_buff)
        .expect("Failed to read line");

    input_buff.chars().nth(0).expect("No character")
}

fn chars_in_string(vc: &Vec<char>, s: &str, guessed_flag: &mut bool) -> String {
    *guessed_flag = true;
    let mut out_string = String::with_capacity(s.len());
    for ch_to_check in s.chars() {
        let mut char_guessed = false;
        for c in vc {
            if ch_to_check == *c {
                char_guessed = true;
                break;
            }
        }
        if !char_guessed {
            out_string.push('_');
            *guessed_flag = false;
        } else {
            out_string.push(ch_to_check);
        }
    }

    out_string
}

fn main() {
    let string_to_guess = setup();
    let mut num_wrong_guesses = 5 as isize;

    let mut correct_chars = vec!['\0'; 26];
    let mut correct_char_idx = 0;

    // kept for checking you haven't already guessed character
    let mut wrong_chars = vec!['\0'; 26];
    let mut wrong_char_idx = 0;

    let mut word_guessed = false;

    while num_wrong_guesses >= 0 {
        let feedback = chars_in_string(&correct_chars, &string_to_guess,
                                       &mut word_guessed);
        println!("{}", feedback);

        if word_guessed {
            println!("You won!");
            break;
        }

        let guess = get_char_input();

        let index = index_in_string(guess, &string_to_guess);

        if index == -1 {
            println!("Nope, you have {} guesses remaining", num_wrong_guesses);
            wrong_chars[wrong_char_idx] = guess;
            wrong_char_idx += 1;
            num_wrong_guesses -= 1;
        } else {
            correct_chars[correct_char_idx] = guess;
            correct_char_idx += 1;
        }
    }

    if !word_guessed {
        println!("The word was {}", string_to_guess);
    }
}
