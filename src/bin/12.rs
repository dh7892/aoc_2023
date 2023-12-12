advent_of_code::solution!(12);

use itertools::Itertools;
use std::collections::HashMap;

fn possible_ways(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    s: &[u8],
    within: usize,
    remaining: &[usize],
) -> usize {
    if s.is_empty() {
        return match (within, remaining.len()) {
            (0, 0) => 1,
            (x, 1) if x == remaining[0] => 1,
            _ => 0,
        };
    }
    if within > 0 && remaining.is_empty() {
        return 0;
    }

    let key = (s.len(), within, remaining.len());
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let ways = match (s[0], within) {
        (b'.', 0) => possible_ways(cache, &s[1..], 0, remaining),
        (b'.', x) if x != remaining[0] => 0,
        (b'.', _) => possible_ways(cache, &s[1..], 0, &remaining[1..]),
        (b'#', 0) => possible_ways(cache, &s[1..], 1, remaining),
        (b'#', x) => possible_ways(cache, &s[1..], x + 1, remaining),
        (b'?', 0) => {
            possible_ways(cache, &s[1..], 1, remaining)
                + possible_ways(cache, &s[1..], 0, remaining)
        }
        (b'?', x) => {
            let mut ans = possible_ways(cache, &s[1..], x + 1, remaining);
            if x == remaining[0] {
                ans += possible_ways(cache, &s[1..], 0, &remaining[1..])
            }
            ans
        }
        _ => unreachable!(),
    };
    cache.insert(key, ways);
    ways
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cache = HashMap::new();
    let result = input
        .lines()
        .map(|l| {
            let (vents, rest) = l.split_once(' ').unwrap();
            let nums = rest
                .split(',')
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            cache.clear();
            let p1 = possible_ways(&mut cache, vents.as_bytes(), 0, &nums);
            p1 as u32
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cache = HashMap::new();
    let result = input
        .lines()
        .map(|l| {
            let (vents, rest) = l.split_once(' ').unwrap();
            let nums = rest
                .split(',')
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let new_vents = (0..5).map(|_| vents).join("?");
            let new_nums = (0..5).flat_map(|_| &nums).copied().collect::<Vec<_>>();
            cache.clear();
            let p2 = possible_ways(&mut cache, new_vents.as_bytes(), 0, &new_nums);
            p2 as u64
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
