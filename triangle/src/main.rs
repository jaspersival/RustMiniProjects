use clap::{ArgMatches, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Number of * elements in the top row
    top_number: usize,
}

#[derive(clap::Subcommand)]
enum Action {
    Top,
    Bottom
}

fn main() {
    let matches = build_cli();
    match matches.subcommand() {
        Some(("top", matches)) => get_top_triangle(get_number(matches)),
        Some(("bottom", matches)) => get_bottom_triangle(get_number(matches)),
        _ => unreachable!("clap should ensure we don't get here")
    };
}

fn build_cli() -> ArgMatches {
    let cmd = clap::Command::new("triangle").bin_name("triangle")
        .subcommand_required(true)
        .subcommand(clap::command!("top").arg(
            clap::Arg::new("NUMBER")
                .value_parser(clap::value_parser!(usize))
        ))
        .subcommand(clap::command!("bottom").arg(
            clap::Arg::new("NUMBER")
                .value_parser(clap::value_parser!(usize))
        ));
    let matches = cmd.get_matches();
    matches
}

fn get_number(matches: &clap::ArgMatches) -> &usize {
    let number = matches.get_one::<usize>("NUMBER").expect("Please provide a NUMBER");
    number
}

fn get_top_triangle(number: &usize) {
    let mut i = *number;
    while i > 0 {
        print_triangle(i);
        i -= 1;
    }
}

fn get_bottom_triangle(number: &usize) {
    let mut i = 1;
    while i <= *number {
        print_triangle(i);
        i += 1;
    }
}

fn print_triangle(i: usize) {
    let row = "*".repeat(i);
    println!("{}", row);
}
