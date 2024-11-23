use std::io::BufRead;

use crate::common::{Config, Part};

mod quest01;
mod quest02;
mod quest03;
mod quest04;

pub const NUM_QUESTS: usize = 4;

pub fn solve(
    quest_number: usize,
    part: Part,
    input: impl BufRead,
    config: &Config,
) -> color_eyre::Result<String> {
    match quest_number {
        1 => quest01::solve(part, input, config),
        2 => quest02::solve(part, input, config),
        3 => quest03::solve(part, input, config),
        4 => quest04::solve(part, input, config),
        _ => Err(color_eyre::eyre::eyre!(
            "That quest has not been solved yet."
        )),
    }
}
