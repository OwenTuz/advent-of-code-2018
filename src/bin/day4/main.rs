extern crate util;
extern crate regex;
use std::collections::BTreeMap;
use regex::Regex;

// The tests got a bit too large today so they've moved to a separate file
#[cfg(test)]
mod test;

#[derive(Debug,PartialEq)]
struct Activity {
    id: i32,
    minute: i32,
    action: Action
}

#[derive(Debug,PartialEq)]
enum Action {
    StartShift,
    Wake,
    Sleep,
    ParsingError,
}

fn parse_input(input: Vec<&str>) -> Vec<Activity> {
    let mut sorted_input = input.to_vec();
    sorted_input.sort();

    let mut id: i32 = -1;
    let id_re = Regex::new("Guard\\s#([0-9]*)\\s").unwrap();
    let minute_re = Regex::new("[0-9][0-9]:([0-9][0-9])\\]").unwrap();

    let mut activities = Vec::new();

    for line in sorted_input {
        let minute = minute_re.captures(line).unwrap()
                              .get(1).unwrap()
                              .as_str().parse::<i32>().unwrap();
        let action = match line {
            it if it.contains("wakes up") => Action::Wake,
            it if it.contains("falls asleep") => Action::Sleep,
            it if it.contains("begins shift") => Action::StartShift,
            _ => Action::ParsingError,
        };
        match action {
            Action::ParsingError => panic!("Parsing error: {}", line),
            Action::StartShift   => {
                id = id_re.captures(line).unwrap()
                                         .get(1).unwrap()
                                         .as_str().parse::<i32>().unwrap();
            },
            _                    => (),
        }
        activities.push(Activity {id: id, minute: minute, action: action})
    }
    activities
}

fn map_sleep_times(input: Vec<Activity>)
    -> std::collections::BTreeMap<i32, std::collections::BTreeMap<i32,i32>>
{
    let mut sleep_times = BTreeMap::new();

    let mut first_min_asleep: i32 = -1;
    let mut last_min_asleep: i32;

    for activity in input {
        let id = &activity.id;
        sleep_times.entry(*id).or_insert(BTreeMap::new());
        sleep_times.get_mut(id).unwrap().entry(99).or_insert(0);

        match activity.action {
            Action::Sleep => first_min_asleep = activity.minute,
            Action::Wake  => {
                last_min_asleep = activity.minute;
                for i in first_min_asleep..last_min_asleep {
                    *sleep_times.get_mut(&activity.id).unwrap()
                               .entry(i).or_insert(0) += 1;
                    *sleep_times.get_mut(&activity.id).unwrap()
                                .get_mut(&99).unwrap() += 1;
                }
            },
            _ => (),
        }
    }
    sleep_times
}

fn main() {
    let raw_input = include_str!("input");
    let activities = parse_input(util::input_string_to_str_vec(raw_input));
    println!("Day1: Answer is:");
}
