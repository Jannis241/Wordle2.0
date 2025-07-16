use std::collections::{HashMap, HashSet};

use crate::game::{Random, Wordle, Color};



#[derive(Clone, Debug)]
struct Slot {
    position: usize,
    possible_letters: Vec<char>,
}

impl Slot {
    pub fn new(position: usize)  -> Self {
        let mut possible_letters = Vec::new();

        for ch in 'a'..='z' {
            possible_letters.push(ch);
        }

        Slot {
            position,
            possible_letters,
        }
    }
}

pub fn get_best_word(wordle: &Wordle) -> String {
    let possible_solutions: Vec<String> = Vec::new();

    let mut slots = Vec::new();

    for i in 0..6 {
        slots.push(Slot::new(i));
    }

    let mut green_letters: HashMap<usize, char> = HashMap::new();
    let mut yellow_letters: HashMap<usize, char> = HashMap::new();
    let mut grey_letters: HashMap<usize, char> = HashMap::new();

    for guess in wordle.words.clone() {
        for ch in guess {
            if ch.color == Color::GREEN {
                green_letters.insert(ch.position, ch.content);
            }
            if ch.color == Color::YELLOW {
                yellow_letters.insert(ch.position, ch.content);
            }
            if ch.color == Color::GREY {
                grey_letters.insert(ch.position, ch.content);
            }
        }
    }

    for (grey_letter_index,grey_letter) in grey_letters {
        for (slot_index, slot) in slots.clone().iter().enumerate() {
            for (letter_index, letter) in slot.possible_letters.iter().enumerate() {
                if letter == &grey_letter {
                    slots[slot_index].possible_letters.remove(letter_index);
                }
            }
        }
    }   

    for (green_letter_index,green_char) in green_letters {
        slots[green_letter_index].possible_letters = vec![green_char]; // Der einzige possible letter ist der Grüne Char.
    }   

    for (yellow_letter_index, yellow_char) in yellow_letters {
        for (slot_index, slot) in slots.clone().iter().enumerate() {
            for (letter_index, letter) in slot.possible_letters.iter().enumerate() {
                if letter == &yellow_char {
                    slots[slot_index].possible_letters.remove(letter_index);
                }
            }
        }
    }


    println!("{:?}", slots);

    return wordle.solutions.get_random_item().clone();
}
