#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use std::{fs, collections::HashMap};

lazy_static! {
    #[derive(Debug)]
    static ref PRIORITY: HashMap<char, usize> = {
        let mut map = HashMap::new();
        ('a'..='z').chain('A'..='Z').enumerate().for_each(|(i, v)| { map.insert(v, i + 1); });
        map
    };
}

fn main() {
    let input_path = "input.txt";
    let input = fs::read_to_string(input_path).unwrap();
    let ln = input.split("\n").filter(|s| !s.is_empty());
    // part 1
    let ln_split = ln.clone().map(|l| { l.split_at(l.len() / 2) });
    let duplicate_char = ln_split.map(|l| l.0.chars().find(|c| { l.1.contains(*c) }).unwrap());
    let priority_sum = duplicate_char.map(|c| PRIORITY.get(&c).unwrap()).sum::<usize>();
    println!("Part 1: {}", priority_sum);
    // part 2
    let group_chunk = ln.chunks(3);
    let group = group_chunk.into_iter().map(|g| { g.collect::<Vec<&str>>() });
    let duplicate_char = group.map(|g| g[0].chars().find(|c| { g[1].contains(*c) && g[2].contains(*c) }).unwrap());
    let priority_sum_2 = duplicate_char.map(|c| PRIORITY.get(&c).unwrap()).sum::<usize>();
    println!("Part 2: {}", priority_sum_2);
}
