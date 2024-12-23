use std::str::FromStr;

use itertools::Itertools;

advent_of_code::solution!(2);

#[derive(Debug)]
struct Report {
    values: Vec<i32>,
}

#[derive(Debug)]
enum Monotony {
    Increasing,
    Decreasing,
    MNone,
}

struct ProblemDampening {
    errored: Option<Problem>,
}

#[derive(Debug, Clone, Copy)]
struct Problem {
    idx: usize,
}

impl ProblemDampening {
    pub fn new() -> ProblemDampening {
        ProblemDampening { errored: None }
    }

    pub fn handle_problem(&mut self, idx: usize) -> Result<(), Problem> {
        if let Some(problem) = self.errored {
            Result::Err(problem)
        } else {
            println!("dampened");
            self.errored = Some(Problem { idx });
            Result::Ok(())
        }
    }

    pub fn would_error_out(&self) -> bool {
        self.errored.is_some()
    }
}

impl Report {
    pub fn is_safe(&self) -> bool {
        use Monotony::*;
        let mut monotony: Monotony = MNone;

        let mut problem_dampening = ProblemDampening::new();

        for x in (1..=2) {
            if x == 2 && !problem_dampening.would_error_out() {
                return true;
            }
            if x == 2 {
                eprintln!("run {x}");
            }
            'inner: for w in self
                .values
                .iter()
                .enumerate()
                .filter(|(i, _)| match problem_dampening.errored {
                    Some(p) => p.idx != *i,
                    None => true,
                })
                .collect_vec()
                .windows(2)
            {
                if let &[(i, a), (_, b)] = w {
                    match &monotony {
                        Increasing if a >= b => {
                            println!("decr after incr");
                            if problem_dampening.handle_problem(i).is_err() {
                                return false;
                            }
                            break 'inner;
                        }
                        Decreasing if a <= b => {
                            println!("incr after decr");
                            if problem_dampening.handle_problem(i).is_err() {
                                return false;
                            }
                            break 'inner;
                        }
                        MNone if a < b => {
                            monotony = Increasing;
                        }
                        MNone if a > b => {
                            monotony = Decreasing;
                        }
                        MNone => {
                            println!("no change");
                            if problem_dampening.handle_problem(i).is_err() {
                                return false;
                            }
                            break 'inner;
                        }
                        _ => {}
                    }

                    // eprintln!("getting abs_diff {}", a.abs_diff(*b));
                    if a.abs_diff(*b) > 3 {
                        println!("diff > 3");
                        if problem_dampening.handle_problem(i).is_err() {
                            return false;
                        }
                        break 'inner;
                    }
                } else {
                    unreachable!();
                }
            }
        }
        true
    }
}

#[derive(Debug)]
struct Err;

impl FromStr for Report {
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s
            .split(' ')
            .map(|s2| s2.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        Ok(Report { values: vec })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();

    let mut reports: Vec<Report> = Vec::new();
    for line in lines.into_iter() {
        reports.push(line.parse::<Report>().unwrap());
    }

    let mut safe_ones = 0;
    for (i, report) in reports.iter().enumerate() {
        if report.is_safe() {
            safe_ones += 1;
        }
    }

    Some(safe_ones)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    // let lines = vec![lines[1]];

    let mut reports: Vec<Report> = Vec::new();
    for line in lines.into_iter() {
        reports.push(line.parse::<Report>().unwrap());
    }

    let mut safe_ones = 0;
    for (i, report) in reports.iter().enumerate() {
        if report.is_safe() {
            safe_ones += 1;
        } else {
            println!("{i} is unsafe");
        }
    }

    Some(safe_ones)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(2));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
