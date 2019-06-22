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

fn main() {
    let input = include_str!("input").trim();
    println!("Part1: Answer is {:?}", react_chain(input).chars().count());
}
