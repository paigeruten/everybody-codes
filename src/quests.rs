use std::io::BufRead;

use crate::common::Part;

mod quest01;
mod quest02;
mod quest03;

pub fn solve(quest_number: usize, part: Part, input: impl BufRead) -> color_eyre::Result<String> {
    match quest_number {
        1 => quest01::solve(part, input),
        2 => quest02::solve(part, input),
        3 => quest03::solve(part, input),
        _ => Err(color_eyre::eyre::eyre!(
            "That quest has not been solved yet."
        )),
    }
}
