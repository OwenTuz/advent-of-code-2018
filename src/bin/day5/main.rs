#[test]
fn test_will_react(){
    assert!(will_react(&'a', &'A'));
    assert_eq!(false, will_react(&'a', &'a'));
    assert_eq!(false, will_react(&'a', &'z'));
    assert_eq!(false, will_react(&'1', &'1'));
}
fn will_react(a: &char, b: &char) -> bool {
    a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

#[test]
fn test_react_chain(){
    assert_eq!("", react_chain(""));
    assert_eq!("", react_chain(""));
    assert_eq!("aabAAB", react_chain("aabAAB"));
    assert_eq!("dabCBAcaDA", react_chain("dabAcCaCBAcCcaDA"));
}
fn react_chain(chain: &str) -> String {
    let mut x: Vec<char> = chain.chars().collect();
    loop {
        let mut i = 0;
        let mut needs_another_pass = false;
        loop {
            if i + 1 >= x.len(){
                break
            }
            let a = x[i];
            let b = x[i + 1];
            if will_react(&a,&b) {
                x.remove(i);
                x.remove(i);
                needs_another_pass = true;
            } else {
                i = i + 1;
            }
        }
        if !needs_another_pass {
            break
        }
    }
    let reacted: String = x.iter().collect();
    reacted

}

#[test]
fn test_find_best_reaction(){
    assert_eq!(4, find_best_reaction("dabAcCaCBAcCcaDA"));
}
fn find_best_reaction(chain: &str) -> usize {
    // Slight cheat - no need to scan through the input and build a character
    // map as we trust our input and know it will be in the set [A-Za-z]
    let lowercase_alphabet = (b'a'..b'z').filter_map(|c| {
        let c = c as char;
        if c.is_alphabetic() {
            Some(c)
        } else { None }
    });
    let mut shortest_found: Option<usize> = None;
    for x in lowercase_alphabet {
        let new_chain: String = chain.chars()
                                     .filter(|c| {
                                        c != &x && c != &x.to_ascii_uppercase()
                                     }).collect();
        let length = react_chain(&new_chain).chars().count();
        if shortest_found == None || length < shortest_found.unwrap() {
            shortest_found = Some(length)
        }
    }
    shortest_found.unwrap()
}


fn main() {
    let input = include_str!("input").trim();
    println!("Part1: Answer is {:?}", react_chain(input).chars().count());
    println!("Part2: Answer is {:?}", find_best_reaction(input));
}
