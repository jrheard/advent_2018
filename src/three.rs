use std::fs;

use serde::Deserialize;
use serde_scan::scan;

#[derive(Deserialize, Debug, PartialEq)]
struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Claim {
    fn new(claim_str: &str) -> Claim {
        let (id, x, y, width, height) = scan!("#{} @ {},{}: {}x{}" <- claim_str).unwrap();
        Claim {
            id,
            x,
            y,
            width,
            height,
        }
    }
}

type FabricGrid = Vec<Vec<i32>>;

fn mark_claim_on_grid(grid: &mut FabricGrid, claim: &Claim) {
    for i in claim.x..(claim.x + claim.width) {
        for j in claim.y..(claim.y + claim.height) {
            grid[i as usize][j as usize] += 1;
        }
    }
}

// How many square inches of fabric are within two or more claims?
pub fn three_a() -> usize {
    let contents = fs::read_to_string("src/inputs/3.txt").unwrap();
    let claims: Vec<Claim> = contents.lines().map(Claim::new).collect();
    let mut grid: FabricGrid = vec![vec![0; 1000]; 1000];

    for claim in &claims {
        mark_claim_on_grid(&mut grid, &claim);
    }

    grid.iter().map(|x| x.iter()).flatten().filter(|x| **x > 1).count()
}

// What is the ID of the only claim that doesn't overlap?
pub fn three_b() -> i32 {
    let contents = fs::read_to_string("src/inputs/3.txt").unwrap();
    let claims: Vec<Claim> = contents.lines().map(Claim::new).collect();
    let mut grid: FabricGrid = vec![vec![0; 1000]; 1000];

    for claim in &claims {
        mark_claim_on_grid(&mut grid, &claim);
    }

    for claim in &claims {
        let mut contested = false;
        for i in claim.x..(claim.x + claim.width) {
            for j in claim.y..(claim.y + claim.height) {
                if grid[i as usize][j as usize] > 1 {
                    contested = true;
                }
            }
        }

        if !contested {
            return claim.id;
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_claim_new() {
        let input = "#123 @ 3,2: 5x4";
        assert_eq!(
            Claim::new(input),
            Claim {
                id: 123,
                x: 3,
                y: 2,
                width: 5,
                height: 4
            }
        );
    }

    #[test]
    fn test_solutions() {
        assert_eq!(three_a(), 101196);
        assert_eq!(three_b(), 243);
    }

}
