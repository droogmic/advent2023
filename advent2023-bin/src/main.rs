use advent2023_lib::{get_days, get_input, DayTrait, Part, PrimaryExample};
use color_eyre::Report;
use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    puzzle: Option<usize>,

    #[structopt(long)]
    all: bool,

    #[structopt(long)]
    parallel: bool,

    #[structopt(long)]
    example: bool,
}

fn print_day<O: std::fmt::Display>(
    day_num: usize,
    display: (&'static str, &'static str),
    result: (O, O),
) {
    println!("Day {}", day_num);
    println!(
        "Part 1: {}",
        display.0.replace("{answer}", &result.0.to_string())
    );
    println!(
        "Part 2: {}",
        display.1.replace("{answer}", &result.1.to_string())
    );
    println!();
}

fn main() -> Result<(), Report> {
    setup()?;

    println!("{}", "Advent Of Code 2022".bold().blue());
    println!();

    let args = Cli::from_args();
    let days = get_days();

    let get_result_pair = move |day_num: usize, day: &Box<dyn DayTrait>| -> (String, String) {
        if args.example {
            match day.get_examples() {
                PrimaryExample::Same(example) => day.both(&example).expect("invalid example"),
                PrimaryExample::Different([first, second]) => (
                    day.calc(Part::First, first).unwrap(),
                    day.calc(Part::Second, second).unwrap(),
                ),
            }
        } else {
            let input = get_input(day_num);
            day.both(&input).expect("invalid input")
        }
    };

    if args.all {
        for (day_num, day) in days.into_iter() {
            print_day(day_num, day.get_display(), get_result_pair(day_num, &day));
        }
    } else if args.parallel {
        let threads = get_days().into_iter().map(|(day_num, day)| {
            println!("Spawn day {}", day_num);
            std::thread::spawn(move || (day_num, day.get_display(), get_result_pair(day_num, &day)))
        });
        std::thread::yield_now();
        std::thread::sleep(std::time::Duration::from_millis(50));
        println!();
        for thread in threads {
            let (day_num, display, (part1, part2)) = thread.join().unwrap();
            print_day(day_num, display, (part1, part2));
        }
    } else if !(args.all || args.parallel) {
        let (day_num, day): (usize, _) = match args.puzzle {
            None => {
                let (last_day_num, last_day) = days.iter().next_back().unwrap();
                (*last_day_num, last_day)
            }
            Some(day_num) => (day_num, days.get(&day_num).unwrap()),
        };
        print_day(day_num, day.get_display(), get_result_pair(day_num, day));
    }

    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1")
    }
    color_eyre::install()?;

    pretty_env_logger::init();
    log::info!("Starting Logging");

    Ok(())
}
