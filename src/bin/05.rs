advent_of_code::solution!(5);

use chumsky::prelude::*;

fn integer_parser() -> impl Parser<char, u64, Error = Simple<char>> {
    text::int(10).map(|s: String| s.parse::<u64>().unwrap())
}

fn integer_list_parser() -> impl Parser<char, Vec<u64>, Error = Simple<char>> {
    integer_parser().separated_by(text::whitespace())
}

/// Structure for representing a range of mappings from source to destination
#[derive(Debug, PartialEq)]
struct MapSpan {
    source: u64,
    target: u64,
    range: u64,
}

impl MapSpan {
    fn destination(&self, source: u64) -> Option<u64> {
        if source >= self.source && source < self.source + self.range {
            Some(self.target + (source - self.source))
        } else {
            None
        }
    }
    fn source(&self, target: u64) -> Option<u64> {
        if target >= self.target && target < self.target + self.range {
            Some(self.source + (target - self.target))
        } else {
            None
        }
    }
}

fn map_span_parser() -> impl Parser<char, MapSpan, Error = Simple<char>> {
    integer_parser()
        .then_ignore(text::whitespace())
        .then(integer_parser())
        .then_ignore(text::whitespace())
        .then(integer_parser())
        .map(|((target, source), range)| MapSpan {
            source,
            target,
            range,
        })
}

#[derive(Debug)]
struct GardenMapping {
    name: String,
    spans: Vec<MapSpan>,
}

impl GardenMapping {
    fn new() -> Self {
        Self {
            name: String::new(),
            spans: Vec::new(),
        }
    }

    fn add_span(&mut self, map_span: MapSpan) {
        self.spans.push(map_span);
    }

    fn destination(&self, source: u64) -> u64 {
        for span in &self.spans {
            if let Some(destination) = span.destination(source) {
                return destination;
            }
        }
        // If we don't have an explicit mapping, the target is the same as the source
        source
    }
    fn source(&self, target: u64) -> u64 {
        for span in self.spans.iter() {
            if let Some(source) = span.source(target) {
                return source;
            }
        }
        // If we don't have an explicit mapping, the target is the same as the source
        target
    }
}

struct SeedRanges {
    ranges: Vec<(u64, u64)>,
}

impl SeedRanges {
    fn contains(&self, value: u64) -> bool {
        for (start, length) in &self.ranges {
            if value >= *start && value < *start + *length {
                return true;
            }
        }
        false
    }
}

fn parse_input_maps(input: &str) -> (Vec<u64>, Vec<GardenMapping>) {
    let mut mappings = Vec::new();
    let mut mapping = GardenMapping::new();

    let mut lines = input.lines();
    let seeds_parser = just("seeds: ").then(integer_list_parser());
    let seeds_line = lines.next().unwrap();
    let seeds = seeds_parser.parse(seeds_line).unwrap().1;

    lines.next();

    for line in lines {
        if line.is_empty() {
            mappings.push(mapping);
            mapping = GardenMapping::new();
        } else if line.contains("map:") {
            mapping.name = line.to_string();
        } else {
            let map_span = map_span_parser().parse(line).unwrap();
            mapping.add_span(map_span);
        }
    }

    mappings.push(mapping);

    (seeds, mappings)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, mappings) = parse_input_maps(input);
    let result = seeds
        .iter()
        .map(|seed| {
            let mut source = *seed;
            for mapping in &mappings {
                let destination = mapping.destination(source);
                source = destination;
            }
            source
        })
        .min();
    Some(result.unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, mappings) = parse_input_maps(input);
    let seed_ranges = SeedRanges {
        ranges: seeds.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect(),
    };

    // It's too slow to try all the seeds. So we work back from locations until we find
    // the lowest location number that routes back to a seed.
    let result = (1..).find(|&seed| {
        let mut destination = seed;
        for mapping in mappings.iter().rev() {
            let source = mapping.source(destination);
            destination = source;
        }
        seed_ranges.contains(destination)
    });

    Some(result.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_map_span_destination() {
        let map_span = MapSpan {
            source: 50,
            target: 98,
            range: 3,
        };

        // Test with a source value before the range
        assert_eq!(map_span.destination(49), None);

        // Test with a source value in the range
        assert_eq!(map_span.destination(50), Some(98));
        assert_eq!(map_span.destination(51), Some(99));
        assert_eq!(map_span.destination(52), Some(100));

        // Test with a source value outside the range
        assert_eq!(map_span.destination(53), None);
    }

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (seeds, mappings) = parse_input_maps(&input);
        assert_eq!(seeds, vec![79, 14, 55, 13]);
        assert_eq!(mappings.len(), 7);
        assert_eq!(mappings[0].name, "seed-to-soil map:");
        assert_eq!(mappings[0].spans.len(), 2);
        assert_eq!(mappings[0].spans[0].target, 50);
        assert_eq!(mappings[0].spans[0].source, 98);
        assert_eq!(mappings[0].spans[0].range, 2);
        assert_eq!(mappings[0].spans[1].target, 52);
        assert_eq!(mappings[0].spans[1].source, 50);
        assert_eq!(mappings[0].spans[1].range, 48);
        assert_eq!(mappings[6].name, "humidity-to-location map:");
        assert_eq!(mappings[6].spans.len(), 2);
        assert_eq!(mappings[6].spans[0].target, 60);
        assert_eq!(mappings[6].spans[0].source, 56);
        assert_eq!(mappings[6].spans[0].range, 37);
        assert_eq!(mappings[6].spans[1].target, 56);
        assert_eq!(mappings[6].spans[1].source, 93);
        assert_eq!(mappings[6].spans[1].range, 4);
    }

    #[test]
    fn test_reverse_mapping() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (_seeds, mappings) = parse_input_maps(&input);

        let mut destination = 35;
        for mapping in mappings.iter().rev() {
            let source = mapping.source(destination);
            destination = source;
        }
        assert_eq!(destination, 13)
    }
}
