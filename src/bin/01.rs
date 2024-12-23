use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();

    println!("{}", lines[0]);

    let mut vl: Vec<u32> = Vec::new();
    let mut vr: Vec<u32> = Vec::new();

    for line in lines {
        let parts = line.split("   ").collect::<Vec<&str>>();
        match parts.as_slice() {
            [a, b] => {
                vl.push(a.parse::<u32>().unwrap());
                vr.push(b.parse::<u32>().unwrap());
            }
            _ => {
                println!("huh");
            }
        }
    }

    vl.sort();
    vr.sort();

    let mut sum = 0;
    for (a, b) in vl.iter().zip(vr.iter()) {
        sum += b.abs_diff(*a);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut index: HashMap<u32, u32> = HashMap::new();

    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();

    let mut vl: Vec<u32> = Vec::new();
    let mut vr: Vec<u32> = Vec::new();

    for line in lines {
        let parts = line.split("   ").collect::<Vec<&str>>();
        match parts.as_slice() {
            [a, b] => {
                vl.push(a.parse::<u32>().unwrap());
                vr.push(b.parse::<u32>().unwrap());
            }
            _ => {
                unreachable!();
            }
        }
    }

    let mut total = 0;

    for l in vl.iter() {
        if let Some(c) = index.get(l) {
            total += l * c;
        }
        let mut count = 0;
        for r in vr.iter() {
            if l == r {
                count += 1;
            }
        }
        index.insert(*l, count);
        total += l * count;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
