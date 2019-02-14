use hashbrown::HashMap;

pub fn frequencies<I, T>(x: I) -> HashMap<T, u32>
where
    I: Iterator<Item = T>,
    T: Copy + Eq + std::hash::Hash,
{
    let mut ret = HashMap::new();

    for item in x {
        let count = ret.entry(item).or_insert(0);
        *count += 1;
    }

    ret
}

pub fn most_common<I, T>(x: I) -> T
where
    I: Iterator<Item = T>,
    T: Copy + Eq + std::hash::Hash,
{
    let freqs = frequencies(x);
    *(freqs.iter().max_by_key(|(_, count)| *count).unwrap().0)
}

#[cfg(test)]
mod test {
    use super::*;

    // via https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal
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
}
