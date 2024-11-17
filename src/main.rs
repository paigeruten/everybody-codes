use color_eyre::eyre::eyre;

use common::{file_reader, Part, Quest};

mod common;
mod quest01;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    if cfg!(debug_assertions) || cfg!(test) {
        std::env::set_var("RUST_BACKTRACE", "full");
    }

    let args: Vec<String> = std::env::args().collect();

    let quest_number = args
        .get(1)
        .expect("Please pass the quest number you want to run")
        .parse::<usize>()
        .expect("Quest number must be an integer");
    let quest = match quest_number {
        1 => quest01::Quest01,
        _ => return Err(eyre!("That quest has not been solved yet.")),
    };

    let part_number = args.get(2).map(|num| {
        num.parse::<usize>()
            .expect("Part number must be an integer")
    });
    let parts = if let Some(part_number) = part_number {
        vec![part_number.try_into()?]
    } else {
        vec![Part::One, Part::Two, Part::Three]
    };

    let input_path = args.get(3);

    println!("\x1b[1mQuest {quest_number}\x1b[0m");
    for part in parts {
        let input = file_reader(
            &input_path
                .cloned()
                .unwrap_or_else(|| part.default_input_path(quest_number)),
        )?;
        let solution = quest.solve(part, input)?;
        println!("  Part {part}: {solution}");
    }

    Ok(())
}