use itertools::Itertools;
use std::{fs, ops::RangeInclusive};

#[derive(Debug, Clone)]
struct Pair(RangeInclusive<usize>, RangeInclusive<usize>);

impl Pair {
    fn new(t: (RangeInclusive<usize>, RangeInclusive<usize>)) -> Self {
        Self(t.0, t.1)
    }

    fn from_str_tup(s: (&str, &str), delimiter: char) -> Result<Self, &'static str> {
        Ok(Self::new(match [s.0, s.1].iter().map(|x| {
            let y = match x.split_once(delimiter) {
                None => return Err("failed to split str"),
                Some(v) => v
            };
            // why does this need to be wrapped in a Result
            // and immediately unwrapped with a map?????
            Ok(RangeInclusive::new(
                match y.0.parse::<usize>() {
                    Err(_) => return Err("parse error"),
                    Ok(v) => v
                },
                match y.1.parse::<usize>() {
                    Err(_) => return Err("parse error"),
                    Ok(v) => v
                }
            ))
        // this unwrap will never panic
        }).map(|v| v.unwrap()).next_tuple() {
            None => return Err("failed to create tuple"),
            Some(v) => v
        }))
    }
}

fn main() {
    // part 1
    let input_path = "input.txt";
    let input = fs::read_to_string(input_path).unwrap();
    let lines = input.split("\n").filter(|s| !s.is_empty());
    let pairs = lines.map(|line| Pair::from_str_tup(line.split_once(',').unwrap(), '-').unwrap());
    // let mut tmp = pairs.clone();
    // tmp.next().unwrap().0.into_inner()
    //pairs.map(|pair| 
    println!("{:?}", pairs.collect::<Vec<Pair>>());
}
