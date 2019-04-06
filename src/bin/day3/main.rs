use std::num::ParseIntError;
use std::str::FromStr;
use std::collections::HashMap;

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

#[test]
fn test_overlapping_claims_total_sq_inches(){
    let claims = vec![
        Claim {id: 1, x: 1, y: 3, width: 4, height: 4},
        Claim {id: 2, x: 3, y: 1, width: 4, height: 4},
        Claim {id: 3, x: 5, y: 5, width: 2, height: 2},
    ];
    assert_eq!(4, overlapping_sq_inches(&claims));
}
fn overlapping_sq_inches(claims: &Vec<Claim>) -> i32 {
    let grid = populate_grid(claims);
    grid.values().filter(|x| **x > 1).count() as i32
}

fn populate_grid(claims: &Vec<Claim>) -> HashMap<(i32, i32), i32> {
    let mut grid = HashMap::new();
    for claim in claims {
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                *grid.entry((x,y)).or_insert(0) += 1;
            }
        }
    }
    grid
}

#[test]
fn test_find_non_overlapping_claim(){
    let claims = vec![
        Claim {id: 1, x: 1, y: 3, width: 4, height: 4},
        Claim {id: 2, x: 3, y: 1, width: 4, height: 4},
        Claim {id: 3, x: 5, y: 5, width: 2, height: 2},
    ];
    assert_eq!(3, find_non_overlapping_claim(&claims));
}
fn find_non_overlapping_claim(claims: &Vec<Claim>) -> i32 {
    let grid = populate_grid(&claims);
    for claim in claims {
        let mut overlapping = 0;
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                if *grid.get(&(x,y)).unwrap() > 1 {
                    overlapping += 1;
                }
            }
        }
        if overlapping == 0 {
            return claim.id
        }
    }
    panic!("NO ANSWER FOUND");
}

fn main() {
    let claims = include_str!("input").trim()
                                      .split("\n")
                                      .map(|x| x.parse::<Claim>().unwrap())
                                      .collect::<Vec<Claim>>();

    println!("Part1: Answer is {}", overlapping_sq_inches(&claims));
    println!("Part2: Answer is {}", find_non_overlapping_claim(&claims));
}
