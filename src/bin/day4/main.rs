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

type GuardFreqMap = std::collections::BTreeMap<i32, std::collections::BTreeMap<i32,i32>>;

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
    -> GuardFreqMap
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

fn find_sleepiest_guard(guard_activities: &GuardFreqMap) -> (i32, i32, i32) {

    let mut highest_mins_asleep: i32 = 0;
    let mut sleepiest_guard_id: i32 = 0;

    for guard_id in guard_activities.keys() {
        let total_mins_slept = *guard_activities.get(&guard_id).unwrap()
                                               .get(&99).unwrap();
        if total_mins_slept > highest_mins_asleep {
            sleepiest_guard_id = *guard_id;
            highest_mins_asleep = total_mins_slept;
        }
    }
    let sleep_times = guard_activities.get(&sleepiest_guard_id).unwrap();
    let sleepiest_minute = get_sleepiest_minute(sleep_times);

    (sleepiest_guard_id, highest_mins_asleep, sleepiest_minute)
}

fn get_sleepiest_minute(freq_map: &std::collections::BTreeMap<i32,i32>) -> i32 {
    let mut cur_high: i32 = 0;
    let mut sleepiest_minute: i32 = 0;
    for i in 0..59 {
        match freq_map.get(&i) {
            Some(x) if x > &cur_high => {
                sleepiest_minute = i;
                cur_high = *x;
            },
            _ => {},
        }
    }
    sleepiest_minute
}

fn part1(sleep_times: &GuardFreqMap) -> i32 {
    let (id,
         _total_mins_slept, // not used but nice to know
         sleepiest_minute) =  find_sleepiest_guard(sleep_times);
    return id * sleepiest_minute
}

fn part2(sleep_times: &GuardFreqMap) -> i32 {
    let mut high_score: i32 = 0;
    let mut winning_id: i32 = 0;
    let mut winning_minute: i32 =0;

    for (id, freq_map) in sleep_times.iter() {
        let candidate_min = get_sleepiest_minute(freq_map);
        match freq_map.get(&candidate_min) {
            Some(x) if x > &high_score => {
                high_score = *x;
                winning_minute = candidate_min;
                winning_id = *id;
            },
            _ => {},
        }
    }
    return winning_id * winning_minute
}

fn main() {
    let raw_input = include_str!("input");
    let activities = parse_input(util::input_string_to_str_vec(raw_input));
    let sleep_times = map_sleep_times(activities);

    println!("Day1: Answer is: {}", part1(&sleep_times));
    println!("Day2: Answer is: {}", part2(&sleep_times));
}
