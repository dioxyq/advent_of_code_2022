use evalexpr::*;
use std::{fs, collections::HashMap};

fn expand_expression(expression: &str, map: &HashMap<&str, &str>) -> String {
    let mut expression = String::from(expression);
    let mut expression_expanded: String;
    loop {
        expression_expanded = expression.split_whitespace()
            .map(|term| format!("{} ", match map.get(term) {
                Some(v) => format!("( {} )", v),
                None => term.to_string()
            })).collect();
        if expression == expression_expanded { break; }
        expression = expression_expanded;
    }
    expression
}

fn get_exprs(equality: (&str, &str), map: &HashMap<&str, &str>, marker: &str) -> (String, String) {
    let mut map = map.clone();
    *map.get_mut("humn").unwrap() = marker;
    let expression_1 = expand_expression(equality.0, &map);
    let expression_2 = expand_expression(equality.1, &map);
    (expression_1, expression_2)
}

fn get_eval(expr: &str, marker: &str, value: usize) -> f64 {
    eval_number(&expr.replace(marker, &value.to_string())).unwrap()
}

fn get_evals(exprs: (&str, &str), marker: &str, value: usize) -> (f64, f64) {
    (get_eval(exprs.0, marker, value), get_eval(exprs.1, marker, value))
}

fn part_2_brute_force(equality: (&str, &str), map: &HashMap<&str, &str>) -> usize {
    let marker = "marker";
    let exprs = get_exprs(equality, map, marker);
    let mut i = 0;
    loop {
        let evals = get_evals((&exprs.1, &exprs.0), marker, i);
        if evals.0 == evals.1 { break; }
        println!("{}", i);
        i += 1;
    }  
    i
}

fn part_2_binary_search(equality: (&str, &str), map: &HashMap<&str, &str>, log: bool) -> usize {
    let marker = "marker";
    let exprs = get_exprs(equality, map, marker);
    let mut lower_bound = 0;
    let mut upper_bound = 1;
    let mut init_search = true;
    let mut log_step = 0;
    loop {
        // evaluate expression
        let evals = get_evals((&exprs.1, &exprs.0), marker, upper_bound);
        // logging
        if log {
            println!("Step: {}\nU: {}\nL: {}\n0: {}\n1: {}\n", log_step, upper_bound, lower_bound, evals.0, evals.1);
            log_step += 1;
        }
        // exit when the correct number (upper_bound) is chosen
        if evals.0 == evals.1 { break; }
        // find the first bounds
        if init_search {
            init_search = evals.0 * 2_f64 < evals.1;
            lower_bound = upper_bound;
            upper_bound *= 2;
            continue;
        }
        // search between bounds
        let diff = upper_bound - lower_bound;
        if evals.0 < evals.1 {
            lower_bound = upper_bound;
            upper_bound += diff / 2;
            continue;
        }
        upper_bound -= diff / 2;
    }  
    upper_bound
}

fn main() {
    let input_path = "input.txt";
    let input = fs::read_to_string(input_path).unwrap();
    let lines = input.split("\n").filter(|s| !s.is_empty());
    let map = HashMap::<&str, &str>::from_iter(lines.map(|line| line.split_once(": ").unwrap()));
    let root = map.get("root").unwrap().clone();

    // part 1
    let expression = expand_expression(root.clone(), &map);
    let eval = eval_number(&expression).unwrap();
    println!("Part 1: {}", eval);

    // part 2
    let equality = root.split_once(" + ").unwrap();
    // let value = part_2_brute_force(equality, &map);
    let value = part_2_binary_search(equality, &map, true);
    println!("Part 2: {}", value);
}
