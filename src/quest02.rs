use std::{collections::HashSet, io::BufRead};

use crate::common::Part;

pub fn solve(part: Part, input: impl BufRead) -> color_eyre::Result<String> {
    let (wordlist, inscription) = parse_input(input)?;
    Ok(match part {
        Part::One => count_words(&wordlist, &inscription).to_string(),
        Part::Two => count_matching_chars(&wordlist, &inscription).to_string(),
        Part::Three => count_matching_chars_in_grid(&wordlist, &inscription).to_string(),
    })
}

fn parse_input(mut input: impl BufRead) -> color_eyre::Result<(Vec<String>, String)> {
    let mut line = String::new();

    input.read_line(&mut line)?;
    if !line.starts_with("WORDS:") {
        return Err(color_eyre::eyre::eyre!("Expected 'WORDS:'"));
    }
    let wordlist: Vec<String> = line[6..]
        .trim_ascii_end()
        .split(',')
        .map(|word| word.to_string())
        .collect();

    if input.read_line(&mut line)? != 1 {
        return Err(color_eyre::eyre::eyre!("Expected blank line"));
    }

    let mut inscription = String::new();
    while input.read_line(&mut inscription)? > 0 {}

    Ok((wordlist, inscription))
}

fn count_words(wordlist: &[String], inscription: &str) -> usize {
    let mut count = 0;
    for (idx, _) in inscription.char_indices() {
        for word in wordlist.iter() {
            let idx_end = idx + word.len();
            if idx_end <= inscription.len() && word[..] == inscription[idx..idx_end] {
                count += 1;
            }
        }
    }
    count
}

fn count_matching_chars(wordlist: &[String], inscription: &str) -> usize {
    // Add reversed words to wordlist
    let wordlist: Vec<String> = wordlist
        .iter()
        .flat_map(|word| [word.clone(), word.chars().rev().collect()])
        .collect();

    let mut matching_indices = HashSet::new();

    for (idx_start, _) in inscription.char_indices() {
        for word in wordlist.iter() {
            let idx_end = idx_start + word.len();
            if idx_end <= inscription.len() && word[..] == inscription[idx_start..idx_end] {
                for idx in idx_start..idx_end {
                    matching_indices.insert(idx);
                }
            }
        }
    }

    matching_indices.len()
}

fn count_matching_chars_in_grid(wordlist: &[String], inscription: &str) -> usize {
    let grid: Vec<Vec<char>> = inscription
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    let mut matching_indices = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            for direction in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                for word in wordlist {
                    let word_matches = word.chars().enumerate().all(|(idx, char)| {
                        let (cur_x, cur_y) = directional_index(x, y, direction, idx, width);
                        (0..height).contains(&cur_y) && grid[cur_y as usize][cur_x as usize] == char
                    });

                    if word_matches {
                        for idx in 0..word.chars().count() {
                            matching_indices.insert(directional_index(x, y, direction, idx, width));
                        }
                    }
                }
            }
        }
    }

    matching_indices.len()
}

fn directional_index(x: i32, y: i32, direction: (i32, i32), idx: usize, width: i32) -> (i32, i32) {
    let (dx, dy) = direction;
    (
        (x + dx * (idx as i32)).rem_euclid(width),
        y + dy * (idx as i32),
    )
}

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::common::{file_reader, Part};
    use std::io::Cursor;

    #[test]
    fn solve_part_one_example() {
        let input = Cursor::new(concat!(
            "WORDS:THE,OWE,MES,ROD,HER\n",
            "\n",
            "AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE\n"
        ));
        assert_eq!("4", solve(Part::One, input).unwrap());
    }

    #[test]
    fn solve_part_one() {
        let input = file_reader("notes/q02p01").unwrap();
        assert_eq!("33", solve(Part::One, input).unwrap());
    }

    #[test]
    fn solve_part_two_example() {
        let input = Cursor::new(concat!(
            "WORDS:THE,OWE,MES,ROD,HER,QAQ\n",
            "\n",
            "AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE\n",
            "THE FLAME SHIELDED THE HEART OF THE KINGS\n",
            "POWE PO WER P OWE R\n",
            "THERE IS THE END\n",
            "QAQAQ\n",
        ));
        assert_eq!("42", solve(Part::Two, input).unwrap());
    }

    #[test]
    fn solve_part_two() {
        let input = file_reader("notes/q02p02").unwrap();
        assert_eq!("5280", solve(Part::Two, input).unwrap());
    }

    #[test]
    fn solve_part_three_example() {
        let input = Cursor::new(concat!(
            "WORDS:THE,OWE,MES,ROD,RODEO\n",
            "\n",
            "HELWORLT\n",
            "ENIGWDXL\n",
            "TRODEOAL\n",
        ));
        assert_eq!("10", solve(Part::Three, input).unwrap());
    }

    #[test]
    fn solve_part_three() {
        let input = file_reader("notes/q02p03").unwrap();
        assert_eq!("11667", solve(Part::Three, input).unwrap());
    }
}
