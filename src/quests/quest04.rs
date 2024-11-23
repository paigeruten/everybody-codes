use std::io::BufRead;

use crate::common::{Config, Part};

pub fn solve(part: Part, input: impl BufRead, _config: &Config) -> color_eyre::Result<String> {
    let solution = match part {
        Part::One | Part::Two => count_strikes(input)?,
        Part::Three => count_bidirectional_strikes(input)?,
    };
    Ok(format!("{solution}"))
}

fn count_strikes(input: impl BufRead) -> color_eyre::Result<i32> {
    let mut minimum: Option<i32> = None;
    let mut total = 0;
    let mut num_nails = 0;
    for line in input.lines() {
        let nail_length: i32 = line?.parse()?;
        if minimum.is_none_or(|min| nail_length < min) {
            minimum = Some(nail_length);
        }
        total += nail_length;
        num_nails += 1;
    }
    Ok(total - minimum.unwrap() * num_nails)
}

fn count_bidirectional_strikes(input: impl BufRead) -> color_eyre::Result<i32> {
    let mut nails: Vec<i32> = input
        .lines()
        .map(|line| line.unwrap().as_str().parse().unwrap())
        .collect();

    nails.sort();
    let median = nails[nails.len() / 2];

    Ok(nails.iter().map(|nail| (nail - median).abs()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::file_reader;
    use std::io::Cursor;

    const CONFIG: Config = Config::test();

    #[test]
    fn solve_part_one_example() {
        let input = Cursor::new("3\n4\n7\n8\n");
        assert_eq!("10", solve(Part::One, input, &CONFIG).unwrap());
    }

    #[test]
    fn solve_part_one() {
        let input = file_reader("notes/q04p01").unwrap();
        assert_eq!("80", solve(Part::One, input, &CONFIG).unwrap());
    }

    #[test]
    fn solve_part_two() {
        let input = file_reader("notes/q04p02").unwrap();
        assert_eq!("824608", solve(Part::Two, input, &CONFIG).unwrap());
    }

    #[test]
    fn solve_part_three_example() {
        let input = Cursor::new("2\n4\n5\n6\n8\n");
        assert_eq!("8", solve(Part::Three, input, &CONFIG).unwrap());
    }

    #[test]
    fn solve_part_three() {
        let input = file_reader("notes/q04p03").unwrap();
        assert_eq!("122004276", solve(Part::Three, input, &CONFIG).unwrap());
    }
}
