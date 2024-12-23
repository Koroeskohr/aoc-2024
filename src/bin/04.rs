use itertools::Itertools;

advent_of_code::solution!(4);

fn a_to_b<I, const N: usize>(mut it: I) -> Option<[char; N]>
where
    I: Iterator<Item = char>,
{
    let x: [Option<char>; N] = [(); N].map(|_| it.next());

    let mut x2: [char; N] = [' '; N];
    for (i, xx) in x.into_iter().enumerate() {
        match xx {
            Some(c) => x2[i] = c,
            None => return None,
        }
    }

    Some(x2)
}

fn get_chars<const N: usize>(s: &str, skip: usize) -> Option<[char; N]> {
    let chars = s.chars().skip(skip).take(N);

    a_to_b(chars)
}

// This is what I wanted to make optimized but it's hard so fuck that
pub fn convert<T, const N: usize>(arr: [Option<T>; N]) -> Option<[T; N]> {
    let arr = arr.into_iter().collect::<Option<Vec<T>>>()?;
    Some(
        arr.try_into()
            .unwrap_or_else(|_| panic!("the array is of size {N}")),
    )
}

fn n_by_n_window<const N: usize>(
    slice: &[&str],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
) -> Option<[[char; N]; N]> {
    if x + N > width || y + N > height {
        return None;
    }

    let mut i = 0;
    let bbb: [Option<[char; N]>; N] = [(); N].map(|_| {
        let idx = i;
        i += 1;
        get_chars::<N>(slice[y + idx], x)
    });

    let out: Option<[[char; N]; N]> = convert(bbb);

    out
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect_vec();
    for x in 0..(140 - 4) {
        for y in 0..(140 - 4) {
            let window: Option<[[char; 4]; 4]> =
                n_by_n_window::<4>(lines.as_slice(), 140, 140, x, y);
            if window.is_none() {
                eprintln!("none!");
            }
        }
    }

    //

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_by_n() {
        let str: Vec<&str> = "asdfg\nzxcvb\nqwert\nyuiop\nfghjk".split('\n').collect();
        let window: Option<[[char; 4]; 4]> = n_by_n_window::<4>(str.as_slice(), 5, 5, 0, 0);

        assert_eq!(
            window,
            Some([
                ['a', 's', 'd', 'f'],
                ['z', 'x', 'c', 'v'],
                ['q', 'w', 'e', 'r'],
                ['y', 'u', 'i', 'o'],
            ])
        )
    }
    #[test]
    fn test_n_by_n_2() {
        let str: Vec<&str> = "asdfg\nzxcvb\nqwert\nyuiop\nfghjk".split('\n').collect();
        let window: Option<[[char; 4]; 4]> = n_by_n_window::<4>(str.as_slice(), 5, 5, 1, 1);

        assert_eq!(
            window,
            Some([
                ['x', 'c', 'v', 'b'],
                ['w', 'e', 'r', 't'],
                ['u', 'i', 'o', 'p'],
                ['g', 'h', 'j', 'k']
            ])
        )
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
