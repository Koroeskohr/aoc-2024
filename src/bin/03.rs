use itertools::Itertools;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut mults: Vec<(i32, i32)> = Vec::new();
    let rgx = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    for (_, [a, b]) in rgx.captures_iter(input).map(|c| c.extract()) {
        eprintln!("got {a} {b}");
        mults.push((a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
    }

    let mut out = 0;
    for (a, b) in mults {
        eprintln!("{a} x {b}");
        out += a * b;
    }

    Some(out as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let x: Vec<&str> = input.split("don't()").collect_vec();
    let (first, rest) = x.split_at(1);
    let first = first[0];

    let x2: Vec<&str> = rest
        .iter()
        .flat_map(|c| c.split("do()").skip(1))
        .collect_vec();

    let mut xx: Vec<&str> = vec![first];
    xx.extend(x2);
    let input: String = xx.concat();

    eprintln!("{input}");
    let mut mults: Vec<(i32, i32)> = Vec::new();
    let rgx = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    for (_, [a, b]) in rgx.captures_iter(input.as_str()).map(|c| c.extract()) {
        eprintln!("got {a} {b}");
        mults.push((a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
    }

    let mut out = 0;
    for (a, b) in mults {
        eprintln!("{a} x {b}");
        out += a * b;
    }

    Some(out as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(1, 1);
    //     assert_eq!(result, Some(161));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
