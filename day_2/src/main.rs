#[macro_use]
extern crate lazy_static;

use std::{fs,collections::HashMap};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum GameType {
        Rock,
        Paper,
        Scissors
}

impl GameType {
    fn get_outcome(&self, opponent: &GameType) -> GameOutcome {
        if *self == *opponent {
            return GameOutcome::Draw
        } 
        if *GAME_T_BEATS_U.get(self).unwrap() == *opponent {
            return GameOutcome::Win
        }
        GameOutcome::Lose
    }

    fn play(&self, opponent: &GameType) -> usize {
        let outcome = self.get_outcome(opponent);
        GAME_OUTCOME_POINTS.get(&outcome).unwrap() + GAME_TYPE_POINTS.get(self).unwrap()
    }

    fn get_outcome_move(&self, outcome: &GameOutcome) -> GameType {
        GAME_T_BEATS_U.iter().find_map(|(key, &_val)| (key.get_outcome(self) == *outcome).then(|| key)).unwrap().clone()
    }
}

#[derive(PartialEq, Eq, Hash)]
enum GameOutcome {
    Win,
    Lose,
    Draw
}

lazy_static! {
    static ref GAME_OUTCOME_POINTS: HashMap<GameOutcome, usize> = HashMap::from([
        (GameOutcome::Win, 6),
        (GameOutcome::Draw, 3),
        (GameOutcome::Lose, 0),
   ]);
}

lazy_static! {
    static ref GAME_NAME_TYPE: HashMap<char, GameType> = HashMap::from([
        ('A', GameType::Rock),
        ('B', GameType::Paper),
        ('C', GameType::Scissors),
        ('X', GameType::Rock),
        ('Y', GameType::Paper),
        ('Z', GameType::Scissors),
    ]);
}

lazy_static! {
    static ref GAME_TYPE_POINTS: HashMap<GameType, usize> = HashMap::from([
        (GameType::Rock, 1),
        (GameType::Paper, 2),
        (GameType::Scissors, 3),
   ]);
}

lazy_static! {
    static ref GAME_T_BEATS_U: HashMap<GameType, GameType> = HashMap::from([
        (GameType::Rock, GameType::Scissors),
        (GameType::Paper, GameType::Rock),
        (GameType::Scissors, GameType::Paper),
   ]);
}

// part 2
lazy_static! {
    static ref GAME_NAME_OUTCOME: HashMap<char, GameOutcome> = HashMap::from([
        ('X', GameOutcome::Lose),
        ('Y', GameOutcome::Draw),
        ('Z', GameOutcome::Win),
    ]);
}

fn main() {
    let input_path = "input.txt";
    let input = fs::read_to_string(input_path).unwrap();
    let ln = input.split("\n").filter(|s| !s.is_empty()).map(str::to_string).collect::<Vec<String>>();
    // part 1
    let points = ln.iter().map(|l| {
        let player = GAME_NAME_TYPE.get(&l.chars().nth(2).unwrap()).unwrap();
        let opponent = GAME_NAME_TYPE.get(&l.chars().nth(0).unwrap()).unwrap();
        player.play(opponent)
    }).sum::<usize>();
    println!("{}", points);
    // part 2
    let points_2 = ln.iter().map(|l| {
        let opponent = GAME_NAME_TYPE.get(&l.chars().nth(0).unwrap()).unwrap();
        let outcome = GAME_NAME_OUTCOME.get(&l.chars().nth(2).unwrap()).unwrap();
        let player = opponent.get_outcome_move(&outcome);
        player.play(opponent)
    }).sum::<usize>();
    println!("{:?}", points_2);
}
