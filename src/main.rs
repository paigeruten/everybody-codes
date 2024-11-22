use common::{file_reader, Config, Part};
use quests::{solve, NUM_QUESTS};

mod common;
mod quests;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    if cfg!(debug_assertions) || cfg!(test) {
        std::env::set_var("RUST_BACKTRACE", "full");
    }

    let mut args: Vec<String> = std::env::args().collect();

    let verbose = if let Some(verbose_idx) = args
        .iter()
        .position(|arg| arg == "-v" || arg == "--verbose")
    {
        args.remove(verbose_idx);
        true
    } else {
        false
    };

    let config = Config { verbose };

    let quest_numbers = if args.len() > 1 {
        vec![args[1]
            .parse::<usize>()
            .expect("Quest number must be an integer")]
    } else {
        (1..=NUM_QUESTS).collect()
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

    for &quest_number in quest_numbers.iter() {
        println!("\x1b[1mQuest {quest_number}\x1b[0m");
        for &part in parts.iter() {
            let input = file_reader(
                &input_path
                    .cloned()
                    .unwrap_or_else(|| part.default_input_path(quest_number)),
            )?;
            let solution = solve(quest_number, part, input, &config)?;
            println!("  Part {part}: {solution}");
        }
    }

    Ok(())
}
