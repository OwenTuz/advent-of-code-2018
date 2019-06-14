use super::*;

#[test]
fn test_parse_input() {
    let input = vec![
        "[1518-11-03 00:29] wakes up",
        "[1518-11-01 00:05] falls asleep",
        "[1518-11-01 00:25] wakes up",
        "[1518-11-01 00:30] falls asleep",
        "[1518-11-01 00:00] Guard #10 begins shift",
        "[1518-11-03 00:05] Guard #10 begins shift",
        "[1518-11-02 00:40] falls asleep",
        "[1518-11-01 00:55] wakes up",
        "[1518-11-01 23:58] Guard #99 begins shift",
        "[1518-11-02 00:50] wakes up",
        "[1518-11-03 00:24] falls asleep",
   ];
   let activities = parse_input(input);
   assert_eq!(00, activities[0].minute);
   assert_eq!(10, activities[0].id);
   assert_eq!(Action::StartShift, activities[0].action);

   assert_eq!(29, activities.last().unwrap().minute);
   assert_eq!(10, activities.last().unwrap().id);
   assert_eq!(Action::Wake, activities.last().unwrap().action);
}

#[test]
fn test_map_sleep_times(){
    let input = vec![
        "[1518-11-03 00:29] wakes up",
        "[1518-11-01 00:05] falls asleep",
        "[1518-11-01 00:25] wakes up",
        "[1518-11-01 00:30] falls asleep",
        "[1518-11-01 00:00] Guard #10 begins shift",
        "[1518-11-03 00:05] Guard #10 begins shift",
        "[1518-11-02 00:40] falls asleep",
        "[1518-11-01 00:55] wakes up",
        "[1518-11-01 23:58] Guard #99 begins shift",
        "[1518-11-02 00:50] wakes up",
        "[1518-11-03 00:24] falls asleep",
    ];
    let output = map_sleep_times(parse_input(input));
    println!("{:?}", output);
    assert_eq!(&1, output.get(&99).unwrap()
                         .get(&49).unwrap());
    assert_eq!(&10, output.get(&99).unwrap()
                         .get(&99).unwrap());
    assert_eq!(&2, output.get(&10).unwrap()
                         .get(&24).unwrap());
}
#[test]
fn test_find_sleepiest_guard(){
    let input = vec![
        "[1518-11-03 00:29] wakes up",
        "[1518-11-01 00:05] falls asleep",
        "[1518-11-01 00:25] wakes up",
        "[1518-11-01 00:30] falls asleep",
        "[1518-11-01 00:00] Guard #10 begins shift",
        "[1518-11-03 00:05] Guard #10 begins shift",
        "[1518-11-02 00:40] falls asleep",
        "[1518-11-01 00:55] wakes up",
        "[1518-11-01 23:58] Guard #99 begins shift",
        "[1518-11-02 00:50] wakes up",
        "[1518-11-03 00:24] falls asleep",
    ];
    let guards = map_sleep_times(parse_input(input));
    assert_eq!(find_sleepiest_guard(&guards), (10, 50, 24))
}
