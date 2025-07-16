use std::collections::{HashMap, HashSet};

use crate::game::{Random, Wordle, Color};



pub fn get_best_word(wordle: &Wordle) -> String {
    let mut green_letters: HashMap<usize, char> = HashMap::new();
    let mut yellow_letters: Vec<(usize, char)> = Vec::new();
    let mut grey_letters: HashSet<char> = HashSet::new();

    for ch in wordle.words.iter().flatten() {
        match ch.color {
            Color::GREEN => {
                green_letters.insert(ch.position, ch.content);
            }
            Color::YELLOW => {
                yellow_letters.push((ch.position, ch.content));
            }
            Color::GREY => {
                grey_letters.insert(ch.content);
            }
        }
    }

    let mut possible_solutions = Vec::new();

    'outer: for word in &wordle.solutions {
        let chars: Vec<char> = word.chars().collect();

        for (&pos, &ch) in &green_letters {
            if chars[pos] != ch {
                continue 'outer;
            }
        }

        for &(pos, ch) in &yellow_letters {
            if chars[pos] == ch || !chars.contains(&ch) {
                continue 'outer;
            }
        }

        for &ch in &grey_letters {
            if chars.contains(&ch)
                && !green_letters.values().any(|&g| g == ch)
                && !yellow_letters.iter().any(|&(_, y)| y == ch)
            {
                continue 'outer;
            }
        }

        possible_solutions.push(word.clone());
    }


    return possible_solutions.get_random_item().clone();
}
