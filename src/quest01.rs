use std::io::BufRead;

use crate::common::Part;

pub fn solve(part: Part, input: impl BufRead) -> color_eyre::Result<String> {
    let solution = match part {
        Part::One => count_potions(parse_battles(input, 1)),
        Part::Two => count_potions(parse_battles(input, 2)),
        Part::Three => count_potions(parse_battles(input, 3)),
    };
    Ok(format!("{solution}"))
}

fn count_potions(battles: impl Iterator<Item = Battle>) -> usize {
    battles.map(|battle| battle.potions_needed()).sum()
}

fn parse_battles(input: impl BufRead, chunk_size: usize) -> impl Iterator<Item = Battle> {
    let mut bytes = input.bytes();
    std::iter::from_fn(move || {
        let mut enemies = vec![];
        for _ in 0..chunk_size {
            if let Ok(enemy) = Enemy::try_from(bytes.next()?.unwrap()) {
                enemies.push(enemy);
            }
        }
        Some(Battle::new(enemies))
    })
}

struct Battle {
    enemies: Vec<Enemy>,
}

impl Battle {
    pub fn new(enemies: Vec<Enemy>) -> Self {
        Self { enemies }
    }

    pub fn potions_needed(&self) -> usize {
        let base_potions_needed: usize = self
            .enemies
            .iter()
            .map(|enemy| enemy.potions_needed())
            .sum();

        let num_enemies = self.enemies.len();
        if num_enemies < 2 {
            base_potions_needed
        } else {
            base_potions_needed + num_enemies * (num_enemies - 1)
        }
    }
}

enum Enemy {
    AncientAnt,
    BadassBeetle,
    CreepyCockroach,
    DiabolicalDragonfly,
}

#[derive(Debug)]
struct UnknownEnemy;

impl TryFrom<u8> for Enemy {
    type Error = UnknownEnemy;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' => Ok(Self::AncientAnt),
            b'B' => Ok(Self::BadassBeetle),
            b'C' => Ok(Self::CreepyCockroach),
            b'D' => Ok(Self::DiabolicalDragonfly),
            _ => Err(UnknownEnemy),
        }
    }
}

impl Enemy {
    pub fn potions_needed(&self) -> usize {
        match self {
            Self::AncientAnt => 0,
            Self::BadassBeetle => 1,
            Self::CreepyCockroach => 3,
            Self::DiabolicalDragonfly => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::common::{file_reader, Part};
    use std::io::Cursor;

    #[test]
    fn solve_part_one_example() {
        let input = Cursor::new(b"ABBAC");
        assert_eq!("5", solve(Part::One, input).unwrap());
    }

    #[test]
    fn solve_part_one() {
        let input = file_reader("notes/q01p01").unwrap();
        assert_eq!("1328", solve(Part::One, input).unwrap());
    }

    #[test]
    fn solve_part_two_example() {
        let input = Cursor::new(b"AxBCDDCAxD");
        assert_eq!("28", solve(Part::Two, input).unwrap());
    }

    #[test]
    fn solve_part_two() {
        let input = file_reader("notes/q01p02").unwrap();
        assert_eq!("5626", solve(Part::Two, input).unwrap());
    }

    #[test]
    fn solve_part_three_example() {
        let input = Cursor::new(b"xBxAAABCDxCC");
        assert_eq!("30", solve(Part::Three, input).unwrap());
    }

    #[test]
    fn solve_part_three() {
        let input = file_reader("notes/q01p03").unwrap();
        assert_eq!("27565", solve(Part::Three, input).unwrap());
    }
}
