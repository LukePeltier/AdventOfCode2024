mod solutions;
mod utils;

use clap::Parser;
use utils::get_input;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short = 'd', long = "day")]
    day: u8,

    #[arg(short = 'b', long = "bonus", default_value_t = false)]
    bonus: bool,
}

fn main() {
    let args = Args::parse();
    match args.day {
        1 => {
            if args.bonus {
                solutions::day01::bonus(&get_input(1).unwrap());
            } else {
                solutions::day01::solve(&get_input(1).unwrap());
            }
        }
        2 => {
            if args.bonus {
                solutions::day02::bonus(&get_input(2).unwrap());
            } else {
                solutions::day02::solve(&get_input(2).unwrap());
            }
        }
        3 => {
            if args.bonus {
                solutions::day03::bonus(&get_input(3).unwrap());
            } else {
                solutions::day03::solve(&get_input(3).unwrap());
            }
        }
        4 => {
            if args.bonus {
                solutions::day04::bonus(&get_input(4).unwrap());
            } else {
                solutions::day04::solve(&get_input(4).unwrap());
            }
        }
        _ => println!("Day {} is not yet implemented", args.day),
    }
}
