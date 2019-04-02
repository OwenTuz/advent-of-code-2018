extern crate util;
use std::collections::HashSet;


#[test]
fn test_calculate_frequency() {
    assert_eq!(3, calculate_frequency(&vec![1,1,1]));
    assert_eq!(0, calculate_frequency(&vec![1,1,-2]));
    assert_eq!(-6, calculate_frequency(&vec![-1,-2,-3]));
}

fn calculate_frequency(oscillations: &Vec<i64>) -> i64 {
    oscillations.into_iter().sum()
}

#[test]
fn test_find_revisited_frequency() {
    assert_eq!(0, find_revisited_frequency(&vec![1,-1]));
}
fn find_revisited_frequency(oscillations: &Vec<i64>) -> i64 {
    let mut current_freq: i64 = 0;
    let mut visited = HashSet::new();

    for osc in oscillations.into_iter().cycle() {
        if visited.contains(&current_freq) {
            break;
        }
        visited.insert(current_freq);
        current_freq = current_freq + osc;
    }
    current_freq
}

fn main() {
    let input = util::input_string_to_integer_vec(include_str!("input"));

    println!("Part 1: Answer is {}", calculate_frequency(&input));
    println!("Part 2: Answer is {}", find_revisited_frequency(&input));
}
