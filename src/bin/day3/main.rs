use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug,PartialEq)]
struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[test]
fn test_claim_from_str() {
    assert_eq!(Claim::from_str("#123 @ 4,3: 5x2"),
               Ok(Claim {id: 123, x: 4, y: 3, width: 5, height: 2}));
    assert_eq!(Claim::from_str("#0 @ 9999,3: 65536x2"),
               Ok(Claim {id: 0, x: 9999, y: 3, width: 65536, height: 2}));
}
impl FromStr for Claim {
    type Err = ParseIntError;
    fn from_str(claim_str: &str) -> Result<Self, Self::Err> {
        let split = claim_str.split(|x| "# :,x".contains(x))
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>();
        // assuming claim format was valid then split now looks like this:
        // [<id>, <x>, <y>, <width>, <height>]
        Ok(Claim {
            id: *split.get(0).unwrap(),
            x: *split.get(1).unwrap(),
            y: *split.get(2).unwrap(),
            width: *split.get(3).unwrap(),
            height: *split.get(4).unwrap(),
        })
    }
}

fn main() {
    println!("{:?}", "#123 @ 4,3: 5x2".parse::<Claim>().unwrap());
}
