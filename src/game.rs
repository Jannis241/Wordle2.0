use std::clone;
use std::collections::HashMap;
use colored::*;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::ptr::hash;
use rand::{random_range, rng, Rng};


#[derive(PartialEq, Debug, Clone)]
pub struct Character {
    pub position: usize,
    pub color: Color,
    pub content: char,
}
impl Character {
    pub fn new(position: usize, color: Color, content: char) -> Self {
        Character { position, color, content}
    }
}

fn get_words_from_file(path: &str) -> Vec<String> {

    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(e) => {
            println!("Error occured while trying to open file: {}", e);
            return Vec::new();
        }
    };

    let reader = io::BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        words.push(line);
    }
    return words;
}



#[derive(PartialEq, Debug, Clone)]
pub enum Color {
    GREY,
    YELLOW,
    GREEN,
}
#[derive(Debug, Clone)]
pub struct Wordle {
    pub words: Vec<Vec<Character>>,
    pub possible_words: Vec<String>,
    pub solutions: Vec<String>,
    solution: String,
    pub guesses: i32,
    pub game_state: GameState,
}


#[derive(PartialEq, Debug, Clone)]
pub enum GameState {
    LOST,
    WON,
    PLAYING,
}

pub trait Random <T> {
    fn get_random_item (&self) -> &T;
}

impl <T> Random<T> for Vec<T>{
    fn get_random_item(&self) -> &T{
        let mut rng = rand::rng();
        let idx = rng.random_range(0..self.len());
        return self.get(idx).expect("Error: Vector is empty.");
    }
}


impl Wordle {
    pub fn new() -> Self {
        
        let possible_words = get_words_from_file("./words.txt");
        let solutions = get_words_from_file("./solutions.txt");

        let solution = solutions.get_random_item().clone();
        let solution = String::from("close"); 


        let mut words: Vec<Vec<Character>> = Vec::new();

        let game_state = GameState::PLAYING;


        for i in 0..6 {
            let mut new_word = Vec::new();
            for j in 0..5 {
                new_word.push(Character::new(j, Color::GREY, '_'));
            }
            words.insert(i, new_word);
        }

        let guesses = 0;
        
        Wordle {
            game_state,
            words,
            possible_words,
            solutions,
            solution,
            guesses,
        }
    }

    fn get_colored_char(&self, c: &char, color: &Color) -> ColoredString {
            let colored_char = match color {
                Color::GREEN => c.to_string().green(),
                Color::YELLOW => c.to_string().yellow(),
                Color::GREY => c.to_string().dimmed(), // oder .dimmed()
            };

            return colored_char;
    }


    pub fn print_board(&self) {
        println!();

        for word in &self.words {
            print!("|");
            let mut full_word: Vec<ColoredString> = Vec::new();

            for c in word {
                full_word.insert(c.position,self.get_colored_char(&c.content, &c.color));
            }

            for colored_string in full_word {
                print!(" {} |", colored_string);
            }
            println!();
        }
        println!()
    }
    
    pub fn is_valid(&self, input: &String) -> bool {
        self.possible_words.contains(&input.trim().to_ascii_lowercase())
    }

    fn update_words(&mut self, input: &String) {
        let solution_chars: Vec<char> = self.solution.chars().into_iter().collect();
        let input_chars: Vec<char> = input.chars().into_iter().collect();


        let mut potential_yellow: HashMap<usize, char> = HashMap::new();
        let mut solution_chars_left: HashMap<usize, char> = HashMap::new();

        for (i,c) in solution_chars.iter().enumerate() {
            solution_chars_left.insert(i, c.clone());
        }
        for (i,c) in input_chars.iter().enumerate() {
            potential_yellow.insert(i, c.clone());
        }
        
        for (i,c) in input_chars.iter().enumerate() {
                self.words[self.guesses as usize][i] = Character::new(i, Color::GREY, input_chars[i]);
                if !solution_chars.contains(c) {
                    potential_yellow.remove(&i);
                }
        }

        for i in 0..solution_chars.len() {
            if solution_chars[i] == input_chars[i] {
                self.words[self.guesses as usize][i] = Character::new(i, Color::GREEN, input_chars[i]);
                potential_yellow.remove(&i);
                solution_chars_left.remove(&i);
            }
        }

        let mut matched_indices = Vec::new();

        for (i, c) in potential_yellow.iter() {
            for (i2, c2) in solution_chars_left.iter() {
                if c == c2 && !matched_indices.contains(i2) {
                    self.words[self.guesses as usize][*i] = Character::new(*i, Color::YELLOW, *c);
                    matched_indices.push(i2.clone()); 
                    break;
                }
            }
        }
    
    }

    pub fn input_word(&mut self, input: &String) {
        self.update_words(input);

        self.guesses += 1;
        if input == &self.solution {
            self.game_state = GameState::WON;
        }

        else if self.guesses == 6 {
            self.game_state = GameState::LOST;
        }
    }
}
