#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]

use chrono::prelude::{DateTime, TimeZone, Timelike, Utc};
use hashbrown::HashMap;
use hashbrown::HashSet;
use itertools::Itertools;

extern crate serde;

#[macro_use]
extern crate serde_scan;

#[macro_use]
extern crate serde_derive;

use std::fs;

fn frequencies<I, T>(x: I) -> HashMap<T, u32>
where
    I: Iterator<Item = T>,
    T: Eq + std::hash::Hash,
{
    let mut ret = HashMap::new();

    for item in x {
        let count = ret.entry(item).or_insert(0);
        *count += 1;
    }

    ret
}

//*******
//* Day 1
//*******

fn one_a() -> i32 {
    let contents = fs::read_to_string("src/inputs/1.txt").unwrap();
    contents.lines().map(|x| x.parse::<i32>().unwrap()).sum()
}

// You notice that the device repeats the same frequency change list over and over.
// To calibrate the device, you need to find the first frequency it reaches twice.
fn one_b() -> i32 {
    let contents = fs::read_to_string("src/inputs/1.txt").unwrap();
    let reductions = contents
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        // "Note that your device might need to repeat its list of frequency changes many
        // times before a duplicate frequency is found."
        .cycle()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        });

    let mut seen_frequencies: HashSet<i32> = HashSet::new();
    for frequency in reductions {
        // TODO ask peter why i have to do &frequency here.
        // Aren't i32s Copy? Shouldn't that mean that they're fine to pass around?
        if seen_frequencies.contains(&frequency) {
            return frequency;
        } else {
            seen_frequencies.insert(frequency);
        }
    }

    -1
}

//*******
//* Day 2
//*******

// To make sure you didn't miss any, you scan the likely candidate boxes again,
// counting the number that have an ID containing exactly two of any letter and
// then separately counting those with exactly three of any letter. You can multiply
// those two counts together to get a rudimentary checksum and compare it
// to what your device predicts.
fn two_a() -> i32 {
    let contents = fs::read_to_string("src/inputs/2.txt").unwrap();

    let mut num_with_a_letter_that_appears_twice = 0;
    let mut num_with_a_letter_that_appears_thrice = 0;

    for letter_freq_map in contents.lines().map(|line| frequencies(line.chars())) {
        if letter_freq_map.values().any(|&x| x == 2) {
            num_with_a_letter_that_appears_twice += 1;
        }
        if letter_freq_map.values().any(|&x| x == 3) {
            num_with_a_letter_that_appears_thrice += 1;
        }
    }

    num_with_a_letter_that_appears_twice * num_with_a_letter_that_appears_thrice
}

fn differing_character_positions(x: &str, y: &str) -> Vec<usize> {
    x.chars()
        .enumerate()
        .filter(|(i, character)| y.chars().nth(*i).unwrap() != *character)
        .map(|(i, _)| i)
        .collect()
}

// The boxes will have IDs which differ by exactly one character at the same position in both strings.
// What letters are common between the two correct box IDs?
fn two_b() -> String {
    let contents = fs::read_to_string("src/inputs/2.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let (box_a, box_b) = lines
        .iter()
        .combinations(2)
        .map(|pair| (pair[0], pair[1]))
        .find(|(box_a, box_b)| differing_character_positions(box_a, box_b).len() == 1)
        .unwrap();

    let differing_index = differing_character_positions(box_a, box_b)[0];

    let mut ret = String::new();
    for (i, character) in box_a.chars().enumerate() {
        if i != differing_index {
            ret.push(character);
        }
    }

    ret
}

//*******
//* Day 3
//*******

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
fn three_a() -> usize {
    let contents = fs::read_to_string("src/inputs/3.txt").unwrap();
    let claims: Vec<Claim> = contents.lines().map(Claim::new).collect();
    let mut grid: FabricGrid = vec![vec![0; 1000]; 1000];

    for claim in &claims {
        mark_claim_on_grid(&mut grid, &claim);
    }

    grid.iter()
        .map(|x| x.iter())
        .flatten()
        .filter(|x| **x > 1)
        .count()
}

// What is the ID of the only claim that doesn't overlap?
fn three_b() -> i32 {
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

//*******
//* Day 4
//*******

type GuardID = u32;

// TODO - is this deriving all sane?
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum LogEntryKind {
    BeginsShift(GuardID),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct LogEntry {
    dt: DateTime<Utc>,
    kind: LogEntryKind,
}

impl LogEntry {
    // Takes a string like "[1518-10-18 23:51] Guard #349 begins shift", returns a LogEntry.
    fn new(log_entry_str: &str) -> LogEntry {
        let dt = parse_log_entry_datetime(log_entry_str);

        // TODO - would a match work here?
        let kind = if log_entry_str.contains("begins shift") {
            let relevant_string_portion = log_entry_str
                .chars()
                .skip_while(|&x| x != ']')
                .collect::<String>();
            let trimmed_str = relevant_string_portion.trim();
            let guard_id = scan!("] Guard #{} begins shift" <- trimmed_str).unwrap();
            LogEntryKind::BeginsShift(guard_id)
        } else if log_entry_str.contains("falls asleep") {
            LogEntryKind::FallsAsleep
        } else {
            LogEntryKind::WakesUp
        };

        LogEntry { dt, kind }
    }
}

// Parses a DateTime<Utc> out of a string like "[1518-10-18 23:51] Guard #349 begins shift".
fn parse_log_entry_datetime(log_entry_str: &str) -> DateTime<Utc> {
    let dt_string = log_entry_str
        .chars()
        .take_while(|&x| x != ']')
        .collect::<String>();
    let dt_str = dt_string.as_str();
    let (year, month, day, hour, minute) = scan!("[{}-{}-{} {}:{}" <- dt_str).unwrap();
    Utc.ymd(year, month, day).and_hms(hour, minute, 0)
}

fn get_guard_sleep_log() -> HashMap<GuardID, Vec<u32>> {
    let contents = fs::read_to_string("src/inputs/4.txt").unwrap();
    let mut entries: Vec<LogEntry> = contents.lines().map(LogEntry::new).collect();

    // Your entries are in the order you found them. You'll need to organize them before they can be analyzed.
    entries.sort();

    // Because all asleep/awake times are during the midnight hour (00:00 - 00:59),
    // only the minute portion (00 - 59) is relevant for those events.
    let mut guard_sleep_log: HashMap<GuardID, Vec<u32>> = HashMap::new();
    let mut current_guard_id = 0;
    let mut sleep_start_minute = 0;

    for entry in &entries {
        match entry.kind {
            LogEntryKind::BeginsShift(guard_id) => {
                current_guard_id = guard_id;
            }
            LogEntryKind::FallsAsleep => {
                sleep_start_minute = entry.dt.minute();
            }
            LogEntryKind::WakesUp => {
                let guard_entry = guard_sleep_log
                    .entry(current_guard_id)
                    .or_insert(Vec::new());
                guard_entry.extend(sleep_start_minute..entry.dt.minute());
            }
        }
    }

    guard_sleep_log
}

// Find the guard that has the most minutes asleep. What minute does that guard spend asleep the most?
fn four_a() -> u32 {
    let guard_sleep_log = get_guard_sleep_log();

    // TODO - better understand when+how+why i have to dereference things in the lines below
    let (sleepiest_guard_id, sleep_minutes) = guard_sleep_log
        .iter()
        .max_by_key(|(_, sleep_minutes)| sleep_minutes.len())
        .unwrap();

    let sleep_minute_frequencies = frequencies(sleep_minutes.iter());
    let (sleepiest_minute, _) = sleep_minute_frequencies
        .iter()
        .max_by_key(|(_, count)| *count)
        .unwrap();

    // What is the ID of the guard you chose multiplied by the minute you chose?
    *sleepiest_guard_id * **sleepiest_minute
}

// Of all guards, which guard is most frequently asleep on the same minute?
fn four_b() -> u32 {
    let guard_sleep_log = get_guard_sleep_log();
    let mut sleepiest_minute_per_guard: HashMap<GuardID, (u32, u32)> = HashMap::new();

    for (&guard_id, sleep_minutes) in &guard_sleep_log {
        let sleep_minute_frequencies = frequencies(sleep_minutes.iter());
        // TODO why do i have to double-deref sleepiest_minute here?
        let (&&sleepiest_minute, &sleep_count_for_minute) = sleep_minute_frequencies
            .iter()
            .max_by_key(|(_, count)| *count)
            .unwrap();

        sleepiest_minute_per_guard.insert(guard_id, (sleepiest_minute, sleep_count_for_minute));
    }

    let (guard_id, (sleepiest_minute, _)) = sleepiest_minute_per_guard
        .iter()
        .max_by_key(|(_, (_, count))| count)
        .unwrap();

    // What is the ID of the guard you chose multiplied by the minute you chose?
    guard_id * sleepiest_minute
}

//*******
//* Day 5
//*******

/// The term "polymer" derives from the Greek word πολύς (polus, meaning "many, much") and μέρος
/// (meros, meaning "part"), and refers to a molecule whose structure is composed of multiple repeating units.

/// The polymer is formed by smaller units which, when triggered, react with each other such that
/// two adjacent units of the same type and opposite polarity are destroyed. Units' types are
/// represented by letters; units' polarity is represented by capitalization. For instance, r and R
/// are units with the same type but opposite polarity, whereas r and s are entirely different types and do not react.
///
/// In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.

/// Returns true if `a` is lowercase, `b` is uppercase, and both are the same letter.
fn polymer_chars_react_one_way_check(a: char, b: char) -> bool {
    a.is_lowercase() && a.to_uppercase().nth(0).unwrap() == b
}

/// Turns "abBA" into "aA".
fn react_polymer_one_step(polymer: &str) -> String {
    let mut ret = String::new();
    let mut prev_char = ' ';

    for character in polymer.chars() {
        let should_destroy = polymer_chars_react_one_way_check(prev_char, character)
            || polymer_chars_react_one_way_check(character, prev_char);
        if should_destroy {
            ret.pop();
            prev_char = ' ';
        } else {
            ret.push(character);
            prev_char = character;
        }
    }

    ret
}

/// Turns "cabBA" into "c".
fn react_polymer(polymer: &str) -> String {
    let mut polymer = polymer.to_string();

    loop {
        let reacted_polymer = react_polymer_one_step(&polymer[..]);
        if polymer == reacted_polymer {
            break;
        } else {
            polymer = reacted_polymer;
        }
    }

    polymer
}

/// How many units remain after fully reacting the polymer you scanned?
fn five_a() -> usize {
    let contents = fs::read_to_string("src/inputs/5.txt").unwrap();
    react_polymer(contents.trim()).len()
}

fn string_without_char(string: &str, character: char) -> String {
    let char_uppercase = character.to_uppercase().nth(0).unwrap();

    string
        .trim()
        .chars()
        .filter(|&char| char != character && char != char_uppercase)
        .collect::<String>()
}

/// One of the unit types is causing problems; it's preventing the polymer from
/// collapsing as much as it should. Your goal is to figure out which unit type
/// is causing the most problems, remove all instances of it (regardless of polarity),
/// fully react the remaining polymer, and measure its length.
fn five_b() -> usize {
    let contents = fs::read_to_string("src/inputs/5.txt").unwrap();
    let contents = contents.trim();
    let mut smallest_length = std::usize::MAX;

    for character in "abcdefghijklmnopqrstuvwxyz".chars() {
        let polymer = string_without_char(contents, character);
        let reacted_polymer = react_polymer(polymer.as_str());
        if reacted_polymer.len() < smallest_length {
            smallest_length = reacted_polymer.len();
        }
    }

    smallest_length
}

fn main() {
    println!("1a: {}", one_a());
    println!("1b: {}", one_b());
    println!("2a: {}", two_a());
    println!("2b: {}", two_b());
    println!("3a: {}", three_a());
    println!("3b: {}", three_b());
    println!("4a: {}", four_a());
    println!("4b: {}", four_b());
    //println!("5a: {}", five_a());
    //println!("5b: {}", five_b());
}

#[cfg(test)]
mod test {
    use super::*;

    // XXX I don't know anything about Rust macros yet, I'm copy-pasting this from
    // https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal for now.
    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
        };
    );

    #[test]
    // A test to ensure that I don't introduce regressions when refactoring.
    fn test_solutions() {
        assert_eq!(one_a(), 439);
        assert_eq!(one_b(), 124645);
        assert_eq!(two_a(), 5368);
        assert_eq!(two_b(), "cvgywxqubnuaefmsljdrpfzyi");
        assert_eq!(three_a(), 101196);
        assert_eq!(three_b(), 243);
        assert_eq!(four_a(), 99911);
        assert_eq!(four_b(), 65854);
        assert_eq!(five_a(), 9900);
        assert_eq!(five_b(), 4992);
    }

    #[test]
    fn test_frequencies() {
        assert_eq!(
            frequencies("aabbccccd".chars()),
            map! { 'a' => 2, 'b' => 2, 'c' => 4, 'd' => 1}
        );

        assert_eq!(
            frequencies("abcabcaa".chars()),
            map! {'a' => 4, 'b' => 2, 'c' => 2}
        );

        assert_eq!(frequencies("".chars()), HashMap::new());
    }

    #[test]
    fn test_differing_character_positions() {
        assert_eq!(differing_character_positions("abcde", "axcye"), vec![1, 3]);
        assert_eq!(differing_character_positions("fghij", "fguij"), vec![2]);
    }

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
    fn test_parse_log_entry_datetime() {
        let dt = parse_log_entry_datetime("[1518-10-18 23:51] Guard #349 begins shift");
        let expected = Utc.ymd(1518, 10, 18).and_hms(23, 51, 0);
        assert_eq!(dt, expected);
    }

    #[test]
    fn test_log_entry_new() {
        let entry = LogEntry::new("[1518-10-18 23:51] Guard #349 begins shift");
        let expected = LogEntry {
            kind: LogEntryKind::BeginsShift(349),
            dt: Utc.ymd(1518, 10, 18).and_hms(23, 51, 0),
        };
        assert_eq!(entry, expected);

        let entry = LogEntry::new("[1518-03-05 00:59] wakes up");
        let expected = LogEntry {
            kind: LogEntryKind::WakesUp,
            dt: Utc.ymd(1518, 3, 5).and_hms(0, 59, 0),
        };
        assert_eq!(entry, expected);

        let entry = LogEntry::new("[1518-04-03 00:19] falls asleep");
        let expected = LogEntry {
            kind: LogEntryKind::FallsAsleep,
            dt: Utc.ymd(1518, 4, 3).and_hms(0, 19, 0),
        };
        assert_eq!(entry, expected);
    }

}
