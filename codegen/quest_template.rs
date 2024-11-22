use std::io::BufRead;

use crate::common::Part;

pub fn solve(part: Part, input: impl BufRead) -> color_eyre::Result<String> {
    let solution = match part {
        Part::One => "todo",
        Part::Two => "todo",
        Part::Three => "todo",
    };
    Ok(format!("{solution}"))
}

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::common::{file_reader, Part};
    use std::io::Cursor;

    #[test]
    #[ignore = "todo"]
    fn solve_part_one_example() {
        let input = Cursor::new(b"");
        assert_eq!("", solve(Part::One, input).unwrap());
    }

    #[test]
    #[ignore = "todo"]
    fn solve_part_one() {
        let input = file_reader("notes/q{{quest_num_padded}}p01").unwrap();
        assert_eq!("", solve(Part::One, input).unwrap());
    }

    #[test]
    #[ignore = "todo"]
    fn solve_part_two_example() {
        let input = Cursor::new(b"");
        assert_eq!("", solve(Part::Two, input).unwrap());
    }

    #[test]
    #[ignore = "todo"]
    fn solve_part_two() {
        let input = file_reader("notes/q{{quest_num_padded}}p02").unwrap();
        assert_eq!("", solve(Part::Two, input).unwrap());
    }

    #[test]
    #[ignore = "todo"]
    fn solve_part_three_example() {
        let input = Cursor::new(b"");
        assert_eq!("", solve(Part::Three, input).unwrap());
    }

    #[test]
    #[ignore = "todo"]
    fn solve_part_three() {
        let input = file_reader("notes/q{{quest_num_padded}}p03").unwrap();
        assert_eq!("", solve(Part::Three, input).unwrap());
    }
}
