use std::fs;

use chrono::prelude::{DateTime, TimeZone, Timelike, Utc};
use hashbrown::HashMap;
use serde_scan::scan;

use super::util;

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
pub fn four_a() -> u32 {
    let guard_sleep_log = get_guard_sleep_log();

    // TODO - better understand when+how+why i have to dereference things in the lines below
    let (sleepiest_guard_id, sleep_minutes) = guard_sleep_log
        .iter()
        .max_by_key(|(_, sleep_minutes)| sleep_minutes.len())
        .unwrap();

    let sleepiest_minute = util::most_common(sleep_minutes.iter());

    // What is the ID of the guard you chose multiplied by the minute you chose?
    *sleepiest_guard_id * sleepiest_minute
}

// Of all guards, which guard is most frequently asleep on the same minute?
pub fn four_b() -> u32 {
    let guard_sleep_log = get_guard_sleep_log();
    let mut sleepiest_minute_per_guard: HashMap<GuardID, (u32, u32)> = HashMap::new();

    for (&guard_id, sleep_minutes) in &guard_sleep_log {
        let sleep_minute_frequencies = util::frequencies(sleep_minutes.iter());
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

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn test_solutions() {
        assert_eq!(four_a(), 99911);
        assert_eq!(four_b(), 65854);
    }

}
