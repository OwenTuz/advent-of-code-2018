extern crate util;
use std::collections::HashMap;

#[test]
fn test_letter_frequencies(){
    assert_eq!(&3, letter_frequencies("aaabbc").get(&'a').unwrap());
    assert_eq!(&2, letter_frequencies("aaabcb").get(&'b').unwrap());
}
fn letter_frequencies(word: &str) -> HashMap<char, i64> {
    let mut freqs = HashMap::new();
    for ch in word.chars() {
        *freqs.entry(ch).or_insert(0) += 1;
    }
    freqs
}

#[test]
fn test_get_checksum(){
    let ids = vec![
        "abcdef",
        "bababc",
        "abbcde",
        "abcccd",
        "aabcdd",
        "abcdee",
        "ababab",
    ];
    assert_eq!(12, get_checksum(&ids));
}
fn get_checksum(ids: &Vec<&str>) -> i64 {
    let mut repeated_twice = 0;
    let mut repeated_three_times = 0;
    for word in ids {
        let freqs = letter_frequencies(word);
        if freqs.values().any(|&x| x == 2) {
            repeated_twice += 1;
        }
        if freqs.values().any(|&x| x == 3) {
            repeated_three_times += 1;
        }
    }
    repeated_twice * repeated_three_times
}

#[test]
fn test_ids_differ_by_exactly_one_char(){
    assert_eq!(true, ids_differ_by_exactly_one_char(&"abcxefg", &"abcdefg"));
    assert_eq!(true, ids_differ_by_exactly_one_char(&"xbcdefg", &"abcdefg"));
    assert_eq!(true, ids_differ_by_exactly_one_char(&"abcdefx", &"abcdefg"));
    assert_eq!(false, ids_differ_by_exactly_one_char(&"abcdefg", &"abcdefg"));
}
fn ids_differ_by_exactly_one_char(word1: &str, word2: &str) -> bool{
    let mut unmatched = 0;
    for (i, ch) in word1.chars().enumerate() {
        if ch != word2.chars().nth(i).unwrap() {
            unmatched += 1;
        }
    }
    return unmatched == 1
}

#[test]
fn test_find_neighbouring_ids(){
    let ids = vec![
        "abcxxg",
        "abcxfx",
        "abcdef",
        "xacxfx",
        "abxdef",
    ];
    assert_eq!("abdef", find_neighbouring_ids(&ids));
}
fn find_neighbouring_ids(ids: &Vec<&str>) -> String {
    for id1 in ids {
        for id2 in ids {
            if ids_differ_by_exactly_one_char(id1, id2) {
                return common_chars(id1, id2)
            }
        }
    }
    panic!("NO ANSWER FOUND")
}

#[test]
fn test_common_chars(){
    assert_eq!("abdef", common_chars(&"abcdef", &"abxdef"));
    assert_eq!("bcdef", common_chars(&"abcdef", &"xbcdef"));
    assert_eq!("abcde", common_chars(&"abcdef", &"abcdex"));
    assert_eq!("bcde", common_chars(&"xbcdef", &"abcdex"));
}
fn common_chars(word1: &str, word2: &str) -> String {
    word1.chars().zip(word2.chars())
                .filter(|pair| pair.0 == pair.1)
                .map(|pair| pair.0)
                .collect::<String>()
}

fn main() {
      let input = util::input_string_to_str_vec(include_str!("input"));
      println!("Part 1: Answer is {}", get_checksum(&input));
      println!("Part 2: Answer is {}", find_neighbouring_ids(&input));
}
