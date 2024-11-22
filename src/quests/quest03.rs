use std::{fmt::Display, io::BufRead};

use crate::common::{Config, Part};

pub fn solve(part: Part, input: impl BufRead, config: &Config) -> color_eyre::Result<String> {
    let mut depth_map = DepthMap::parse(input)?;
    if part == Part::Three {
        depth_map.set_diagonal_neighbours(true);
    }

    if config.verbose {
        println!("{depth_map}");
    }

    depth_map.trim();

    if config.verbose {
        println!("{depth_map}");
    }

    while depth_map.dig() > 0 {
        if config.verbose {
            println!("{depth_map}");
        }
    }

    Ok(format!("{}", depth_map.sum()))
}

#[derive(Debug, PartialEq)]
struct DepthMap {
    map: Vec<Vec<u8>>,
    diagonal_neighbours: bool,
}

impl DepthMap {
    pub fn new(map: Vec<Vec<u8>>) -> Self {
        Self {
            map,
            diagonal_neighbours: false,
        }
    }

    pub fn set_diagonal_neighbours(&mut self, diagonal_neighbours: bool) {
        self.diagonal_neighbours = diagonal_neighbours;
    }

    pub fn parse(mut input: impl BufRead) -> color_eyre::Result<Self> {
        let mut map = Vec::new();

        let mut line = String::new();
        loop {
            line.clear();
            if input.read_line(&mut line)? == 0 {
                break;
            }

            let row: Vec<u8> = line
                .trim_ascii_end()
                .chars()
                .filter_map(|c| match c {
                    '.' => Some(0),
                    '#' => Some(1),
                    _ => None,
                })
                .collect();

            map.push(row);
        }

        Ok(Self::new(map))
    }

    pub fn trim(&mut self) {
        let mut min_x: Option<usize> = None;
        let mut min_y: Option<usize> = None;
        let mut max_x: Option<usize> = None;
        let mut max_y: Option<usize> = None;

        for (y, row) in self.map.iter().enumerate() {
            for (x, &depth) in row.iter().enumerate() {
                if depth > 0 {
                    if min_x.is_none_or(|min_x| x < min_x) {
                        min_x = Some(x);
                    }
                    if min_y.is_none_or(|min_y| y < min_y) {
                        min_y = Some(y);
                    }
                    if max_x.is_none_or(|max_x| x > max_x) {
                        max_x = Some(x);
                    }
                    if max_y.is_none_or(|max_y| y > max_y) {
                        max_y = Some(y);
                    }
                }
            }
        }

        if let (Some(min_x), Some(min_y), Some(max_x), Some(max_y)) = (min_x, min_y, max_x, max_y) {
            let mut trimmed = Vec::new();
            for (y, row) in self.map.iter().enumerate() {
                if min_y <= y && y <= max_y {
                    trimmed.push(row[min_x..=max_x].to_vec());
                }
            }
            self.map = trimmed;
        } else {
            // Map is completely flat - trim it to 1x1 I guess?
            self.map = vec![vec![0]];
        }
    }

    pub fn dig(&mut self) -> usize {
        let snapshot = self.map.clone();
        let mut blocks_dug = 0;
        let neighbours = self.neighbours();

        for (y, row) in snapshot.iter().enumerate() {
            for (x, &depth) in row.iter().enumerate() {
                if depth == 0 {
                    continue;
                }

                let same_neighbours = neighbours.iter().all(|(dx, dy)| {
                    let (nx, ny) = (((x as isize) + dx) as usize, ((y as isize) + dy) as usize);
                    snapshot
                        .get(ny)
                        .is_some_and(|r| r.get(nx).is_some_and(|&neighbour| neighbour == depth))
                });
                if same_neighbours {
                    self.map[y][x] += 1;
                    blocks_dug += 1;
                }
            }
        }
        blocks_dug
    }

    fn neighbours(&self) -> Vec<(isize, isize)> {
        let mut result = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
        if self.diagonal_neighbours {
            result.append(&mut vec![(1, -1), (1, 1), (-1, 1), (-1, -1)]);
        }
        result
    }

    pub fn sum(&self) -> usize {
        self.map
            .iter()
            .map(|row| row.iter().map(|&d| d as usize).sum::<usize>())
            .sum()
    }
}

impl Display for DepthMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.map.iter() {
            for &depth in row.iter() {
                if depth == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{:X}", depth)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::file_reader;
    use std::io::Cursor;

    const CONFIG: Config = Config::test();

    #[test]
    fn depth_map_trim() {
        let mut map = DepthMap::new(vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![0, 0, 0, 1, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
        ]);

        map.trim();

        let expected = DepthMap::new(vec![
            vec![1, 1, 0],
            vec![0, 0, 1],
            vec![0, 0, 1],
            vec![1, 1, 0],
        ]);

        assert_eq!(map, expected);
    }

    #[test]
    fn solve_part_one_example() {
        let input = Cursor::new(concat!(
            "..........\n",
            "..###.##..\n",
            "...####...\n",
            "..######..\n",
            "..######..\n",
            "...####...\n",
            "..........\n"
        ));
        assert_eq!("35", solve(Part::One, input, &CONFIG).unwrap());
    }

    #[test]
    fn solve_part_one() {
        let input = file_reader("notes/q03p01").unwrap();
        assert_eq!("120", solve(Part::One, input, &CONFIG).unwrap());
    }

    #[test]
    fn solve_part_two() {
        let input = file_reader("notes/q03p02").unwrap();
        assert_eq!("2712", solve(Part::Two, input, &CONFIG).unwrap());
    }

    #[test]
    fn solve_part_three_example() {
        let input = Cursor::new(concat!(
            "..........\n",
            "..###.##..\n",
            "...####...\n",
            "..######..\n",
            "..######..\n",
            "...####...\n",
            "..........\n"
        ));
        assert_eq!("29", solve(Part::Three, input, &CONFIG).unwrap());
    }

    #[test]
    fn solve_part_three() {
        let input = file_reader("notes/q03p03").unwrap();
        assert_eq!("10336", solve(Part::Three, input, &CONFIG).unwrap());
    }
}
